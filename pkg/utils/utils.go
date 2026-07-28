package utils

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

// HumanReadableSize formats a byte size into a human readable string.
func HumanReadableSize(bytes int64) string {
	units := []string{"B", "KB", "MB", "GB", "TB"}
	size := float64(bytes)
	idx := 0
	for size >= 1024.0 && idx < len(units)-1 {
		size /= 1024.0
		idx++
	}
	return fmt.Sprintf("%.2f %s", size, units[idx])
}

// MakeDirs creates a directory hierarchy if it doesn't already exist.
func MakeDirs(path string) error {
	return os.MkdirAll(path, 0755)
}

// DeleteAllTempFolders recursively removes all folders named "temp".
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
