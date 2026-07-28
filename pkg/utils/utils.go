package utils

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

// HumanReadableSize formats a byte size into a human readable string (KB, MB, GB, etc.)
func HumanReadableSize(bytes int64) string {
	const unit = 1024.0
	if bytes < 1024 {
		return fmt.Sprintf("%d B", bytes)
	}
	div, exp := int64(unit), 0
	for n := bytes / unit; n >= 1; n /= unit {
		div *= unit
		exp++
	}
	units := []string{"KB", "MB", "GB", "TB"}
	return fmt.Sprintf("%.2f %s", float64(bytes)/float64(div/1024), units[exp])
}

// MakeDirs creates a directory hierarchy if it doesn't already exist.
func MakeDirs(path string) error {
	return os.MkdirAll(path, 0755)
}

// DeleteAllTempFolders recursively searches and removes all folders named "temp".
func DeleteAllTempFolders(rootDir string) error {
	return filepath.Walk(rootDir, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			if os.IsNotExist(err) {
				return nil
			}
			return err
		}
		if info.IsDir() && strings.ToLower(info.Name()) == "temp" {
			os.RemoveAll(path)
			return filepath.SkipDir
		}
		return nil
	})
}
