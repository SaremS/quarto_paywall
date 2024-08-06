package paywall

import (
	"golang.org/x/net/html"
	"strings"
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

func TestAppendHtmlToHtmlNodeWithScript(t *testing.T) {
	htmlDocString := `<html><head></head><body></body></html>`
	htmlInsertString := `<script>console.log("test")</script>`
	appendNode := "body"

	result, err := appendHtmlToHtmlNode(htmlDocString, htmlInsertString, appendNode)

	if err != nil {
		t.Fatalf("appendHtmlToHtmlNode() error = %v", err)
	}

	target := `<html><head></head><body><script>console.log("test")</script></body></html>`
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

func TestGetContentAfterClass(t *testing.T) {
	htmlStr := `<html><head></head><body><div class="test">test</div><div class="test2">test2</div></body></html>`
	className := "test"

	result, err := getContentAfterClass(htmlStr, className)

	if err != nil {
		t.Fatalf("getContentAfterClass() error = %v", err)
	}

	target := `<div class="test2">test2</div>`
	if result != target {
		t.Errorf("getContentAfterClass() = %v, want %v", result, target)
	}
}

func TestReplaceContentAfterClass(t *testing.T) {
	htmlStr := `<html><head></head><body><div class="test">test</div><div class="test2">test2</div></body></html>`
	className := "test"
	replacement := "test3"

	result, err := replaceContentAfterClass(htmlStr, className, replacement)

	if err != nil {
		t.Fatalf("getContentAfterClass() error = %v", err)
	}

	target := `<html><head></head><body><div class="test">test</div>test3</body></html>`

	if result != target {
		t.Errorf("getContentAfterClass() = %v, want %v", result, target)
	}
}

func TestFindNodeByClassAndParent(t *testing.T) {
	htmlStr := `<html><head></head><body><div class="test"></div></body></html>`

	doc, err := html.Parse(strings.NewReader(htmlStr))
	if err != nil {
		t.Fatalf("html.Parse() error = %v", err)
	}

	className := "test"

	node, nodeParent := findNodeByClassAndParent(doc, className, "body")

	if node == nil {
		t.Fatalf("findNodeByClassAndParent() error = %v", err)
	}

	if nodeParent == nil {
		t.Fatalf("findNodeByClassAndParent() error = %v", err)
	}

	if node.Data != "div" {
		t.Errorf("findNodeByClassAndParent() = %v, want %v", node.Data, "div")
	}

	if nodeParent.Data != "body" {
		t.Errorf("findNodeByClassAndParent() = %v, want %v", nodeParent.Data, "body")
	}
}
