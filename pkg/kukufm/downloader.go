package kukufm

import (
	"fmt"
	"net/http"
	"os"
	"path/filepath"
	"strings"
	"sync"

	"kukufm-dl/pkg/models"
	"kukufm-dl/pkg/utils"
)

type KuKuFMDownloader struct {
	Conf           models.InputConf
	KuKuFM         *KuKuFM
	M3U8DL         *M3U8Downloader
	AudioProcessor *AudioProcessor
	DownloadPath   string
}

func NewKuKuFMDownloader(conf models.InputConf, httpClient, httpDLClient *http.Client, downloadPath string) *KuKuFMDownloader {
	return &KuKuFMDownloader{
		Conf:           conf,
		KuKuFM:         NewKuKuFM(httpDLClient),
		M3U8DL:         NewM3U8Downloader(httpClient, httpDLClient),
		AudioProcessor: NewAudioProcessor(httpDLClient),
		DownloadPath:   downloadPath,
	}
}

func (d *KuKuFMDownloader) DownloadEpisode(ep models.Episode) error {
	fmt.Printf("Downloading episode %s\n", ep.Title)

	playlistURL, err := d.M3U8DL.GetPlaylistURL(ep.HLSURL)
	if err != nil {
		return fmt.Errorf("failed to get playlist url: %w", err)
	}

	streamURLs, err := d.M3U8DL.GetStreamURLs(playlistURL)
	if err != nil {
		return fmt.Errorf("failed to get stream urls: %w", err)
	}

	showDir := filepath.Join(d.DownloadPath, ep.ShowTitle)
	if err := utils.MakeDirs(showDir); err != nil {
		return err
	}

	tempDir := filepath.Join(showDir, "temp")
	if err := utils.MakeDirs(tempDir); err != nil {
		return err
	}

	cleanTitle := strings.ReplaceAll(ep.Title, "/", "-")
	cleanShowTitle := strings.ReplaceAll(ep.ShowTitle, "/", "-")
	episodeFile := fmt.Sprintf("%s - %s.m4a", cleanShowTitle, cleanTitle)

	type streamResult struct {
		idx  int
		path string
		err  error
	}

	results := make([]streamResult, len(streamURLs))
	var wg sync.WaitGroup
	sem := make(chan struct{}, 10) // concurrent downloads for segments

	for idx, url := range streamURLs {
		wg.Add(1)
		go func(i int, u string) {
			defer wg.Done()
			sem <- struct{}{}
			defer func() { <-sem }()

			parts := strings.Split(u, "/")
			fileName := parts[len(parts)-1]
			path, err := d.M3U8DL.DownloadStream(u, fmt.Sprintf("%d_%s", i, fileName), tempDir)
			results[i] = streamResult{idx: i, path: path, err: err}
		}(idx, url)
	}
	wg.Wait()

	var streamPaths []string
	for _, res := range results {
		if res.err != nil {
			return fmt.Errorf("segment download failed: %w", res.err)
		}
		streamPaths = append(streamPaths, res.path)
	}

	epFileDir, err := d.AudioProcessor.MergeFilesFFmpeg(streamPaths, episodeFile, showDir)
	if err != nil {
		return fmt.Errorf("merge failed: %w", err)
	}

	fileInfo, err := os.Stat(epFileDir)
	if err == nil {
		fmt.Printf("Downloaded - %s (%s)\n", episodeFile, utils.HumanReadableSize(fileInfo.Size()))
	} else {
		fmt.Printf("Downloaded - %s\n", episodeFile)
	}

	fmt.Printf("Adding metadata to %s\n", episodeFile)
	_ = d.AudioProcessor.AddMetadataToFile(ep, epFileDir)
	fmt.Printf("Metadata added to %s\n", episodeFile)

	return nil
}

func (d *KuKuFMDownloader) Download() error {
	showSlug, err := d.KuKuFM.GetShowSlug(d.Conf.ShowURL)
	if err != nil {
		return err
	}

	episodes, show, err := d.KuKuFM.GetEpisodes(showSlug, d.Conf.FromEp, d.Conf.ToEp)
	if err != nil {
		return err
	}

	fmt.Println()
	fmt.Printf("Show Title: %s\n", show.Title)
	fmt.Printf("Author: %s\n", show.AuthorName)
	fmt.Printf("Description: %s\n", show.Description)
	fmt.Printf("Language: %s\n", show.Language)
	fmt.Printf("Estimated Episodes : ~%d\n", show.NPages*10)
	fmt.Println()

	sem := make(chan struct{}, d.Conf.ParallelDownloads)
	var wg sync.WaitGroup

	for _, ep := range episodes {
		wg.Add(1)
		sem <- struct{}{}
		go func(e models.Episode) {
			defer wg.Done()
			defer func() { <-sem }()

			if err := d.DownloadEpisode(e); err != nil {
				fmt.Printf("Error downloading episode %s: %v\n", e.Title, err)
			}
		}(ep)
	}

	wg.Wait()
	return nil
}
