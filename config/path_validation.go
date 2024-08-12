package config

import (
	"path/filepath"
	"strings"
	"unicode/utf8"
)

func isValidPath(path string) bool {
	// Check if the path is a valid UTF-8 string
	if !utf8.ValidString(path) {
		return false
	}

	// Check for the presence of null characters
	if strings.ContainsRune(path, '\x00') {
		return false
	}

	// Use filepath.Clean to normalize the path
	cleanedPath := filepath.Clean(path)

	// Ensure the cleaned path does not end with a separator and is not empty
	if cleanedPath == "" || strings.HasSuffix(cleanedPath, string(filepath.Separator)) {
		return false
	}

	// Ensure the last element is a filename
	fileName := filepath.Base(cleanedPath)
	if fileName == "." || fileName == "/" || fileName == "\\" {
		return false // Invalid filename (indicates directory or root)
	}

	return true
}
