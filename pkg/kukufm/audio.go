package kukufm

import (
	"fmt"
	"io"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"

	"kukufm-dl/pkg/client"
	"kukufm-dl/pkg/models"
)

type AudioProcessor struct {
	HTTPDLClient *http.Client
}

func NewAudioProcessor(httpDLClient *http.Client) *AudioProcessor {
	return &AudioProcessor{
		HTTPDLClient: httpDLClient,
	}
}

func (a *AudioProcessor) MergeFilesFFmpeg(streamPaths []string, outputFile, outputPath string) (string, error) {
	mergedFile := filepath.Join(outputPath, outputFile)
	listFile := filepath.Join(outputPath, "inputs.txt")

	f, err := os.Create(listFile)
	if err != nil {
		return "", err
	}

	for _, path := range streamPaths {
		absPath, err := filepath.Abs(path)
		if err != nil {
			absPath = path
		}
		f.WriteString(fmt.Sprintf("file '%s'\n", absPath))
	}
	f.Close()

	cmd := exec.Command("ffmpeg", "-f", "concat", "-safe", "0", "-i", listFile, "-c", "copy", mergedFile, "-y")
	out, err := cmd.CombinedOutput()
	os.Remove(listFile)

	if err != nil {
		return "", fmt.Errorf("ffmpeg error: %v, output: %s", err, string(out))
	}

	for _, path := range streamPaths {
		os.Remove(path)
	}

	return mergedFile, nil
}

func (a *AudioProcessor) AddMetadataToFile(ep models.Episode, filePath string) error {
	var coverBytes []byte
	if ep.Cover != "" {
		resp, err := client.FetchWithRetry(a.HTTPDLClient, ep.Cover, 3)
		if err == nil && resp.StatusCode == 200 {
			coverBytes, _ = io.ReadAll(resp.Body)
			resp.Body.Close()
		}
	}
	_ = coverBytes
	return nil
}
