package paywall

import (
	"bytes"
	"fmt"
	log "github.com/go-pkgz/lgr"
	"golang.org/x/net/html"
	"io"
	"strings"
)

func appendNewNodeWithContent(htmlStr string, className string, childItemContent string, data string, key string, val string) (string, error) {
	doc, err := html.Parse(strings.NewReader(htmlStr))
	if err != nil {
		return "", fmt.Errorf("error parsing HTML: %w", err)
	}

	node := findNodeByClass(doc, className)
	if node == nil {
		return htmlStr, nil
	}

	// Create the new <li> node
	newNode := createNodeWithContent(childItemContent, data, key, val)

	if newNode == nil {
		return "", fmt.Errorf("error creating new node")
	}

	// Append the <li> node to the found node
	node.AppendChild(newNode)

	// Render the modified HTML back to a string
	var buf bytes.Buffer
	w := io.Writer(&buf)
	if err := html.Render(w, doc); err != nil {
		return "", fmt.Errorf("error rendering HTML: %w", err)
	}

	return buf.String(), nil
}

func createNodeWithContent(content string, data string, key string, val string) *html.Node {
	liNode := &html.Node{
		Type: html.ElementNode,
		Data: data,
		Attr: []html.Attribute{
			{Key: key, Val: val},
		},
		FirstChild: &html.Node{
			Type: html.RawNode,
			Data: content,
		},
	}

	return liNode
}

func getContentAfterClass(htmlStr string, className string) (string, error) {
	doc, err := html.Parse(strings.NewReader(htmlStr))
	if err != nil {
		return "", fmt.Errorf("error parsing HTML: %w", err)
	}

	node := findNodeByClass(doc, className)
	if node == nil {
		return "", nil
	}

	var contentAfterDiv bytes.Buffer
	for sibling := node.NextSibling; sibling != nil; sibling = sibling.NextSibling {
		if err := html.Render(&contentAfterDiv, sibling); err != nil {
			return "", fmt.Errorf("error rendering content after div: %w", err)
		}
	}

	return contentAfterDiv.String(), nil
}


func findNodeByClassAndParent(n *html.Node, className string, parentClassName string) (*html.Node, *html.Node) {
	node := findNodeByClass(n, className)
	if node == nil {
		return nil, nil
	}

	if node.Parent == nil {
		return node, nil
	}

	return node, node.Parent
}

func findNodeByClass(n *html.Node, className string) *html.Node {
	if n.Type == html.ElementNode {
		for _, attr := range n.Attr {
			if attr.Key == "class" && attr.Val == className {
				return n
			}
		}
	}
	for c := n.FirstChild; c != nil; c = c.NextSibling {
		if result := findNodeByClass(c, className); result != nil {
			return result
		}
	}
	return nil
}

func appendHtmlToHtmlNode(htmlDocString string, htmlInsertString string, appendNode string) (string, error) {

	doc, err := html.Parse(strings.NewReader(htmlDocString))
	if err != nil {
		log.Fatalf("Failed to parse base HTML: %v", err)
		return "", nil
	}

	targetNode := extractTargetNodePointer(doc, appendNode)

	if targetNode == nil {
		return "", fmt.Errorf("target node not found")
	}

	insertDoc, err := html.Parse(strings.NewReader(htmlInsertString))
	if err != nil {
		log.Fatalf("Failed to parse target HTML: %v", err)
		return "", nil
	}

	insertNode := extractTargetNodePointer(insertDoc, "body").FirstChild

	//delete everything around insertNode, since html.Parse() places
	//everything in a <html><head></head><body></body></html> structure
	deleteParentsAndSiblings(insertNode)

	targetNode.AppendChild(insertNode)

	var buf bytes.Buffer
	w := io.Writer(&buf)
	html.Render(w, doc)

	return buf.String(), nil
}

func extractTargetNodePointer(doc *html.Node, targetNodeData string) *html.Node {
	var targetNode *html.Node
	var f func(*html.Node)
	f = func(n *html.Node) {
		if n.Type == html.ElementNode && n.Data == targetNodeData {
			targetNode = n
			return
		}
		for c := n.FirstChild; c != nil; c = c.NextSibling {
			f(c)
		}
	}
	f(doc)

	return targetNode
}

func deleteParentsAndSiblings(node *html.Node) {
	node.Parent = nil
	node.PrevSibling = nil
	node.NextSibling = nil
}
