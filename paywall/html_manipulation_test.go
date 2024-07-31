package paywall

import (
	"testing"
)

func TestAppendHtmlToHtmlNode(t *testing.T) {
	htmlDocString := `<html><head></head><body></body></html>`
	htmlInsertString := `<div>test</div>`
	appendNode := "body"

	result, err := appendHtmlToHtmlNode(htmlDocString, htmlInsertString, appendNode)

	if err != nil {
		t.Fatalf("appendHtmlToHtmlNode() error = %v", err)
	}

	target := "<html><head></head><body><div>test</div></body></html>"
	if result != target {
		t.Errorf("appendHtmlToHtmlNode() = %v, want %v", result, target)
	}
}
