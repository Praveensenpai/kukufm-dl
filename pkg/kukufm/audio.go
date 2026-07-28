package kukufm

import (
	"fmt"
	"io"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"strconv"

	"kukufm-dl/pkg/client"
	"kukufm-dl/pkg/models"
)

type AudioProcessor struct {
	HTTPDLClient *http.Client
}

func NewAudioProcessor(httpDLClient *http.Client) *AudioProcessor {
	return &AudioProcessor{HTTPDLClient: httpDLClient}
}

// MergeFilesFFmpeg concatenates HLS stream segments into a single m4a file via ffmpeg.
func (a *AudioProcessor) MergeFilesFFmpeg(streamPaths []string, outputFile, outputPath string) (string, error) {
	mergedFile := filepath.Join(outputPath, outputFile)
	listFile := filepath.Join(outputPath, "inputs.txt")

	f, err := os.Create(listFile)
	if err != nil {
		return "", fmt.Errorf("create inputs list: %w", err)
	}

	for _, path := range streamPaths {
		absPath, err := filepath.Abs(path)
		if err != nil {
			absPath = path
		}
		if _, err := fmt.Fprintf(f, "file '%s'\n", absPath); err != nil {
			f.Close()
			os.Remove(listFile)
			return "", fmt.Errorf("write inputs list: %w", err)
		}
	}
	f.Close()

	cmd := exec.Command("ffmpeg", "-f", "concat", "-safe", "0", "-i", listFile, "-c", "copy", mergedFile, "-y")
	out, err := cmd.CombinedOutput()
	os.Remove(listFile)

	if err != nil {
		return "", fmt.Errorf("ffmpeg merge: %v — output: %s", err, string(out))
	}

	for _, path := range streamPaths {
		os.Remove(path)
	}

	return mergedFile, nil
}

// AddMetadataToFile embeds episode metadata and cover art into the m4a file using ffmpeg.
func (a *AudioProcessor) AddMetadataToFile(ep models.Episode, filePath string) error {
	tmpFile := filePath + ".tmp.m4a"

	args := []string{
		"-i", filePath,
		"-metadata", "title=" + ep.Title,
		"-metadata", "album=" + ep.ShowTitle,
		"-metadata", "artist=" + ep.Author,
		"-metadata", "comment=" + ep.Description,
		"-metadata", "track=" + strconv.Itoa(ep.No),
		"-metadata", "language=" + ep.Language,
	}

	// Download and embed cover art if available
	if ep.Cover != "" {
		coverPath, err := a.downloadCover(ep.Cover, filepath.Dir(filePath))
		if err == nil {
			defer os.Remove(coverPath)
			args = append(args,
				"-i", coverPath,
				"-map", "0", "-map", "1",
				"-c", "copy",
				"-disposition:v:0", "attached_pic",
			)
		} else {
			args = append(args, "-c", "copy")
		}
	} else {
		args = append(args, "-c", "copy")
	}

	args = append(args, tmpFile, "-y")

	cmd := exec.Command("ffmpeg", args...)
	out, err := cmd.CombinedOutput()
	if err != nil {
		os.Remove(tmpFile)
		return fmt.Errorf("ffmpeg metadata: %v — output: %s", err, string(out))
	}

	if err := os.Rename(tmpFile, filePath); err != nil {
		os.Remove(tmpFile)
		return fmt.Errorf("replace file after metadata: %w", err)
	}

	return nil
}

func (a *AudioProcessor) downloadCover(url, destDir string) (string, error) {
	resp, err := client.FetchWithRetry(a.HTTPDLClient, url, 3)
	if err != nil {
		return "", err
	}
	defer resp.Body.Close()

	if resp.StatusCode != 200 {
		return "", fmt.Errorf("cover fetch status %d", resp.StatusCode)
	}

	coverPath := filepath.Join(destDir, "cover.jpg")
	f, err := os.Create(coverPath)
	if err != nil {
		return "", err
	}
	defer f.Close()

	if _, err := io.Copy(f, resp.Body); err != nil {
		os.Remove(coverPath)
		return "", err
	}

	return coverPath, nil
}
