package models

type InputConf struct {
	ShowURL           string
	FromEp            int
	ToEp              int
	ParallelDownloads int
}

type Show struct {
	Title       string
	Description string
	AuthorName  string
	Language    string
	NPages      int
}

type Episode struct {
	ShowTitle   string
	Title       string
	No          int
	DurationS   int
	HLSURL      string
	Cover       string
	Author      string
	Language    string
	Description string
}
