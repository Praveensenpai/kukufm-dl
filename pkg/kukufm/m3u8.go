package kukufm

import (
	"fmt"
	"io"
	"net/http"
	"net/url"
	"os"
	"path/filepath"
	"sort"

	"kukufm-dl/pkg/client"

	"github.com/grafov/m3u8"
)

type M3U8Downloader struct {
	HTTPClient   *http.Client
	HTTPDLClient *http.Client
}

func NewM3U8Downloader(httpClient, httpDLClient *http.Client) *M3U8Downloader {
	return &M3U8Downloader{
		HTTPClient:   httpClient,
		HTTPDLClient: httpDLClient,
	}
}

func (m *M3U8Downloader) FetchM3U8(playlistURL string) (m3u8.Playlist, error) {
	resp, err := client.FetchWithRetry(m.HTTPClient, playlistURL, 10)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("bad status fetching m3u8: %s", resp.Status)
	}

	p, listType, err := m3u8.DecodeFrom(resp.Body, true)
	if err != nil {
		return nil, err
	}
	_ = listType
	return p, nil
}

func (m *M3U8Downloader) GetPlaylistURL(hlsURL string) (string, error) {
	p, err := m.FetchM3U8(hlsURL)
	if err != nil {
		return "", err
	}

	masterPl, ok := p.(*m3u8.MasterPlaylist)
	if !ok || len(masterPl.Variants) == 0 {
		return hlsURL, nil
	}

	sort.Slice(masterPl.Variants, func(i, j int) bool {
		return masterPl.Variants[i].Bandwidth > masterPl.Variants[j].Bandwidth
	})

	u, err := url.Parse(hlsURL)
	if err != nil {
		return "", err
	}
	refURL, err := url.Parse(masterPl.Variants[0].URI)
	if err != nil {
		return "", err
	}

	return u.ResolveReference(refURL).String(), nil
}

func (m *M3U8Downloader) GetStreamURLs(playlistURL string) ([]string, error) {
	p, err := m.FetchM3U8(playlistURL)
	if err != nil {
		return nil, err
	}

	mediaPl, ok := p.(*m3u8.MediaPlaylist)
	if !ok {
		return nil, fmt.Errorf("expected media playlist, got master")
	}

	u, err := url.Parse(playlistURL)
	if err != nil {
		return nil, err
	}

	var streamURLs []string
	for _, seg := range mediaPl.Segments {
		if seg == nil {
			continue
		}
		ref, err := url.Parse(seg.URI)
		if err != nil {
			continue
		}
		streamURLs = append(streamURLs, u.ResolveReference(ref).String())
	}

	return streamURLs, nil
}

func (m *M3U8Downloader) DownloadStream(url, filename, outputPath string) (string, error) {
	resp, err := client.FetchWithRetry(m.HTTPDLClient, url, 10)
	if err != nil {
		return "", err
	}
	defer resp.Body.Close()

	destPath := filepath.Join(outputPath, filename)
	outFile, err := os.Create(destPath)
	if err != nil {
		return "", err
	}
	defer outFile.Close()

	_, err = io.Copy(outFile, resp.Body)
	if err != nil {
		return "", err
	}

	return destPath, nil
}
