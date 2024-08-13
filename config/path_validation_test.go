package config

import "testing"

func TestIsValidPath(t *testing.T) {
	path := "/test/whatever.html"
	if !isValidPath(path) {
		t.Errorf("validatePath() = false, want true")
	}
}

func TestIsValidPath_InvalidPath(t *testing.T) {
	path := ""
	if isValidPath(path) {
		t.Errorf("validatePath() = true, want false")
	}
}
