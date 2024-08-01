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

func TestAppendNewNodeWithContent(t *testing.T) {
	htmlDocString := `<html><head></head><body><div class="test"></div></body></html>`
	className := "test"
	chileItemContent := "test"
	data := "div"
	key := "class"
	val := "test2"

	result, err := appendNewNodeWithContent(htmlDocString, className, chileItemContent, data, key, val)

	if err != nil {
		t.Fatalf("appendNewNodeWithContent() error = %v", err)
	}

	target := `<html><head></head><body><div class="test"><div class="test2">test</div></div></body></html>`
	if result != target {
		t.Errorf("appendNewNodeWithContent() = %v, want %v", result, target)
	}
}
