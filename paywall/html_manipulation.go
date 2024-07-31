package paywall

import (
	"bytes"
	"golang.org/x/net/html"
	log "github.com/go-pkgz/lgr"
	"io"
	"strings"
)

func appendHtmlToHtmlNode(htmlDocString string, htmlInsertString string, appendNode string) (string, error) {

	doc, err := html.Parse(strings.NewReader(htmlDocString))
	if err != nil {	
		log.Fatalf("Failed to parse base HTML: %v", err)
		return "", nil
	}

	var targetNode *html.Node
	var f func(*html.Node)
	f = func(n *html.Node) {
		if n.Type == html.ElementNode && n.Data == appendNode {
			targetNode = n
			return
		}
		for c := n.FirstChild; c != nil; c = c.NextSibling {
			f(c)
		}
	}
	f(doc)

	insertDoc, err := html.Parse(strings.NewReader(htmlInsertString))
	if err != nil {
		log.Fatalf("Failed to parse target HTML: %v", err)
		return "", nil
	}

	var insertNode *html.Node
	var g func(*html.Node)
	g = func(n *html.Node) {
		if n.Type == html.ElementNode && n.Data == "body" {
			insertNode = n.FirstChild
			return
		}
		for c := n.FirstChild; c != nil; c = c.NextSibling {
			g(c)
		}
	}
	g(insertDoc)
		
	//delete everything around insertNode, since html.Parse() places 
	//everything in a <html><head></head><body></body></html> structure
	insertNode.Parent = nil
	insertNode.PrevSibling = nil
	insertNode.NextSibling = nil

	targetNode.AppendChild(insertNode)

	var buf bytes.Buffer
	w := io.Writer(&buf)
	html.Render(w, doc)

	return buf.String(), nil
}
