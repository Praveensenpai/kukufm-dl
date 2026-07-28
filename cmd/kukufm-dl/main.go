package main

import (
	"flag"
	"fmt"
	"os"
	"strings"

	"kukufm-dl/pkg/client"
	"kukufm-dl/pkg/kukufm"
	"kukufm-dl/pkg/models"
	"kukufm-dl/pkg/utils"
)

func main() {
	url := flag.String("url", "", "Show URL (required)")
	fromEp := flag.Int("from-ep", 1, "Start episode (>= 1)")
	toEp := flag.Int("to-ep", 0, "End episode (0 for infinite)")
	parallelDownloads := flag.Int("parallel-downloads", 1, "Number of parallel downloads")

	flag.Parse()

	if *url == "" || !strings.Contains(*url, "/show/") {
		fmt.Fprintln(os.Stderr, "Error: Invalid parameter --url. URL must contain '/show/'.")
		flag.Usage()
		os.Exit(1)
	}

	if *fromEp < 1 {
		fmt.Fprintln(os.Stderr, "Error: From episode must be at least 1.")
		os.Exit(1)
	}

	if *toEp < 0 {
		fmt.Fprintln(os.Stderr, "Error: To episode must be 0 or more.")
		os.Exit(1)
	}

	if *toEp != 0 && *fromEp > *toEp {
		fmt.Fprintln(os.Stderr, "Error: To episode can't be less than from episode.")
		os.Exit(1)
	}

	if *parallelDownloads < 1 {
		fmt.Fprintln(os.Stderr, "Error: Parallel downloads must be at least 1.")
		os.Exit(1)
	}

	conf := models.InputConf{
		ShowURL:           *url,
		FromEp:            *fromEp,
		ToEp:              *toEp,
		ParallelDownloads: *parallelDownloads,
	}

	downloadPath := "downloads/"
	if err := utils.MakeDirs(downloadPath); err != nil {
		fmt.Fprintf(os.Stderr, "Error creating download directory: %v\n", err)
		os.Exit(1)
	}

	if err := utils.DeleteAllTempFolders(downloadPath); err != nil {
		fmt.Fprintf(os.Stderr, "Error cleaning temp folders: %v\n", err)
	}

	httpClient, err := client.GetClient()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error initializing HTTP client: %v\n", err)
		os.Exit(1)
	}

	httpDLClient := client.GetDLClient()

	dl := kukufm.NewKuKuFMDownloader(conf, httpClient, httpDLClient, downloadPath)
	if err := dl.Download(); err != nil {
		fmt.Fprintf(os.Stderr, "Download error: %v\n", err)
		os.Exit(1)
	}
}
