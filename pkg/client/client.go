package client

import (
	"crypto/tls"
	"fmt"
	"net/http"
	"net/http/cookiejar"
	"os"
	"strings"
	"time"
)

func parseCookiesFile(filepath string) ([]*http.Cookie, error) {
	content, err := os.ReadFile(filepath)
	if err != nil {
		return nil, err
	}
	var cookies []*http.Cookie
	pairs := strings.Split(string(content), ";")
	for _, pair := range pairs {
		pair = strings.TrimSpace(pair)
		if pair == "" {
			continue
		}
		parts := strings.SplitN(pair, "=", 2)
		if len(parts) == 2 {
			cookies = append(cookies, &http.Cookie{
				Name:  strings.TrimSpace(parts[0]),
				Value: strings.TrimSpace(parts[1]),
			})
		}
	}
	return cookies, nil
}

type HeaderTransport struct {
	Transport http.RoundTripper
}

func (h *HeaderTransport) RoundTrip(req *http.Request) (*http.Response, error) {
	req.Header.Set("accept", "*/*")
	req.Header.Set("accept-language", "en")
	req.Header.Set("dnt", "1")
	req.Header.Set("priority", "u=1, i")
	req.Header.Set("sec-ch-ua", `"Not(A:Brand";v="99", "Google Chrome";v="133", "Chromium";v="133"`)
	req.Header.Set("sec-ch-ua-mobile", "?0")
	req.Header.Set("sec-ch-ua-platform", `"Linux"`)
	req.Header.Set("sec-fetch-dest", "empty")
	req.Header.Set("sec-fetch-mode", "cors")
	req.Header.Set("sec-fetch-site", "same-origin")
	req.Header.Set("user-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36")
	return h.Transport.RoundTrip(req)
}

func GetClient() (*http.Client, error) {
	jar, err := cookiejar.New(nil)
	if err != nil {
		return nil, err
	}

	cookies, err := parseCookiesFile("cookies.txt")
	if err == nil && len(cookies) > 0 {
		reqURL, _ := http.NewRequest("GET", "https://kukufm.com", nil)
		jar.SetCookies(reqURL.URL, cookies)
	}

	baseTransport := &http.Transport{
		TLSClientConfig: &tls.Config{InsecureSkipVerify: true},
	}

	return &http.Client{
		Jar:       jar,
		Timeout:   3 * time.Minute,
		Transport: &HeaderTransport{Transport: baseTransport},
	}, nil
}

func GetDLClient() *http.Client {
	return &http.Client{
		Timeout: 3 * time.Minute,
	}
}

func FetchWithRetry(httpClient *http.Client, url string, maxAttempts int) (*http.Response, error) {
	var resp *http.Response
	var err error
	for attempt := 1; attempt <= maxAttempts; attempt++ {
		req, rErr := http.NewRequest("GET", url, nil)
		if rErr != nil {
			return nil, rErr
		}
		resp, err = httpClient.Do(req)
		if err == nil {
			if resp.StatusCode < 500 {
				return resp, nil
			}
			resp.Body.Close()
		}
		time.Sleep(3 * time.Second)
	}
	if err != nil {
		return nil, err
	}
	return nil, fmt.Errorf("failed to fetch %s after %d attempts", url, maxAttempts)
}
