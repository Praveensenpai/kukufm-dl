package kukufm

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"strings"

	"kukufm-dl/pkg/client"
	"kukufm-dl/pkg/models"
)

type KuKuFM struct {
	HTTPDLClient *http.Client
}

func NewKuKuFM(httpDLClient *http.Client) *KuKuFM {
	return &KuKuFM{HTTPDLClient: httpDLClient}
}

type APIAuthor struct {
	Name string `json:"name"`
}

type APIShow struct {
	Title       string    `json:"title"`
	Description string    `json:"description"`
	Author      APIAuthor `json:"author"`
	Language    string    `json:"language"`
}

type APIContent struct {
	HLSURL string `json:"hls_url"`
}

type APIEpisode struct {
	Index     int        `json:"index"`
	Title     string     `json:"title"`
	DurationS int        `json:"duration_s"`
	Content   APIContent `json:"content"`
	Image     string     `json:"image"`
}

type APIShowResponse struct {
	Show     APIShow      `json:"show"`
	NPages   int          `json:"n_pages"`
	Episodes []APIEpisode `json:"episodes"`
}

func (k *KuKuFM) GetShowSlug(showURL string) (string, error) {
	parts := strings.Split(strings.TrimSpace(showURL), "/")
	for i, p := range parts {
		if p == "show" && i+1 < len(parts) {
			return parts[i+1], nil
		}
	}
	return "", fmt.Errorf("Show/Slug not found in URL %s", showURL)
}

func (k *KuKuFM) GetShow(showSlug string) (*models.Show, error) {
	url := fmt.Sprintf("https://kukufm.com/api/v2.1/channels/%s/episodes?lang=english&page=1", showSlug)
	resp, err := client.FetchWithRetry(k.HTTPDLClient, url, 3)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	if resp.StatusCode == 404 {
		return nil, fmt.Errorf("Show/Slug %s not found", showSlug)
	}

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}

	var apiResp APIShowResponse
	if err := json.Unmarshal(body, &apiResp); err != nil {
		return nil, err
	}

	return &models.Show{
		Title:       apiResp.Show.Title,
		Description: apiResp.Show.Description,
		AuthorName:  apiResp.Show.Author.Name,
		Language:    apiResp.Show.Language,
		NPages:      apiResp.NPages,
	}, nil
}

func (k *KuKuFM) GetEpisodes(showSlug string, fromEp, toEp int) ([]models.Episode, *models.Show, error) {
	show, err := k.GetShow(showSlug)
	if err != nil {
		return nil, nil, err
	}

	perPageEp := 10
	currentPage := ((fromEp - 1) / perPageEp) + 1
	var result []models.Episode

	for {
		url := fmt.Sprintf("https://kukufm.com/api/v2.1/channels/%s/episodes?lang=english&page=%d", showSlug, currentPage)
		resp, err := client.FetchWithRetry(k.HTTPDLClient, url, 3)
		if err != nil {
			return nil, nil, err
		}

		if resp.StatusCode == 404 {
			resp.Body.Close()
			return nil, nil, fmt.Errorf("Show/Slug %s not found", showSlug)
		}

		body, err := io.ReadAll(resp.Body)
		resp.Body.Close()
		if err != nil {
			return nil, nil, err
		}

		var apiResp APIShowResponse
		if err := json.Unmarshal(body, &apiResp); err != nil {
			return nil, nil, err
		}

		if len(apiResp.Episodes) == 0 {
			break
		}

		done := false
		for _, ep := range apiResp.Episodes {
			if ep.Index < fromEp {
				continue
			}
			if toEp > 0 && ep.Index > toEp {
				done = true
				break
			}

			result = append(result, models.Episode{
				ShowTitle:   show.Title,
				Title:       ep.Title,
				No:          ep.Index,
				DurationS:   ep.DurationS,
				HLSURL:      ep.Content.HLSURL,
				Cover:       ep.Image,
				Author:      show.AuthorName,
				Language:    show.Language,
				Description: show.Description,
			})
		}

		if done || currentPage >= apiResp.NPages {
			break
		}
		currentPage++
	}

	return result, show, nil
}
