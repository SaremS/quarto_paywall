package main

import (
	"bytes"
	"fmt"
	log "github.com/go-pkgz/lgr"
	"golang.org/x/net/html"
	"html/template"
	"io"
	"io/ioutil"
	"strings"
)

type UserInfo struct {
	Name     string
	LoggedIn bool
}

type PaywallTemplate struct {
	Template       template.Template
	WalledContent  *template.HTML
	LoginwallContent *template.HTML
	PaywallContent *template.HTML
}

type PaywallStatic struct {
	Paywall string
	Registerwall string
	LoginGithub string
}

func LoadPaywallStatic(path string) (*PaywallStatic, error) {
	paywallContent, err := ReadFileToString(path + "/paywall.html")
	if err != nil {
		return nil, err
	}

	registerwallContent, err := ReadFileToString(path + "/registerwall.html")
	if err != nil {
		return nil, err
	}

	loginGithubContent, err := ReadFileToString(path + "/login_github.html")
	if err != nil {
		return nil, err
	}

	return &PaywallStatic{
		Paywall: paywallContent,
		Registerwall: registerwallContent,
		LoginGithub: loginGithubContent,
	}, nil
}

func ReadFileToString(path string) (string, error) {
	data, err := ioutil.ReadFile(path)
	if err != nil {
		return "", err
	}

	return string(data), nil
}


// paywall struct
type Paywall struct {
	tmpl_map map[string]*PaywallTemplate
}


// new paywall from filepath
func NewPaywall(filePath string, filePathLoader RecursiveLoader, paywallStatic *PaywallStatic) *Paywall {
	// iterate over all files in all subfolders; only load html files
	target_map, err := filePathLoader.WalkTarget(filePath, ".html", func(content string) PaywallTemplate {

		content_app, err := appendHtmlToHtmlNode(content, paywallStatic.LoginGithub, "body")

		if err != nil {
			log.Fatalf("failed to fetch and load html: %v", err)
		}

		content_app, err = addLoginListElement(content_app)

		if err != nil {
			log.Fatalf("failed to fetch and load html: %v", err)
		}

		content_final, walled, err_final := extractAndReplaceContent(content_app)

		if err_final != nil {
			log.Fatalf("failed to fetch and load html: %v", err)
		}

		tmpl, err := template.New(filePath).Parse(content_final)

		if err != nil {
			log.Fatalf("failed to parse template: %v", err)
		}

		loginwallContent := paywallStatic.Registerwall
		paywallContent := paywallStatic.Paywall

		var walledHtml *template.HTML

		if walled == nil {
			walledHtml = nil
		} else {
			walledHtmlBefore := template.HTML(*walled)
			walledHtml = &walledHtmlBefore

		}

		loginwallContentHtml := template.HTML(loginwallContent)
		paywallContentHtml := template.HTML(paywallContent)
		

		paywallTemplate := PaywallTemplate{
			Template: *tmpl,
			WalledContent: walledHtml,
			LoginwallContent: &loginwallContentHtml,
			PaywallContent: &paywallContentHtml,
		}

		return paywallTemplate
	})

	if err != nil {
		log.Fatalf("failed to load templates: %v", err)
	}

	//iterate over target_map and create template map
	template_map := make(map[string]*PaywallTemplate)
	for key, value := range target_map {
		tmpl := value


		template_map[key] = tmpl
	}

	return &Paywall{tmpl_map: template_map}
}

func appendHtmlToHtmlNode(htmlDocString string, htmlInsertString string, appendNode string) (string, error) {

	// Parse the HTML string
	doc, err := html.Parse(strings.NewReader(htmlDocString))
	if err != nil {	
		return "", nil
	}
	target, err := html.Parse(strings.NewReader(htmlInsertString))
	if err != nil {
		return "", nil
	}

	// Find the <body> node
	var bodyNode *html.Node
	var f func(*html.Node)
	f = func(n *html.Node) {
		if n.Type == html.ElementNode && n.Data == appendNode {
			bodyNode = n
			return
		}
		for c := n.FirstChild; c != nil; c = c.NextSibling {
			f(c)
		}
	}
	f(doc)

	// Append the <script> node to the <body> node
	bodyNode.AppendChild(target)

	// Render the modified HTML back to a string
	var buf bytes.Buffer
	w := io.Writer(&buf)
	html.Render(w, doc)

	return buf.String(), nil
}

func addLoginListElement(htmlString string) (string, error) {
	targetString := `
		{{ if .LoggedIn }}	
			<button class="nav-link" onclick="runLogout()">Logout</button>
		{{ else }}
			<button class="nav-link" onclick="runLogin()">Login</button>
		{{ end }}`

	result, err := appendListItem(htmlString, "navbar-nav navbar-nav-scroll ms-auto", targetString)

	if err != nil {
		return "", err
	}

	return result, nil
}

// findNodeByClass recursively searches for a node with the specified class name.
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

// appendListItem appends a new <li>somecontent</li> node to the node with the specified class name.
func appendListItem(htmlStr, className, listItemContent string) (string, error) {
	doc, err := html.Parse(strings.NewReader(htmlStr))
	if err != nil {
		return "", fmt.Errorf("error parsing HTML: %w", err)
	}

	node := findNodeByClass(doc, className)
	if node == nil {
		return htmlStr, nil
	}

	// Create the new <li> node
	liNode := &html.Node{
		Type: html.ElementNode,
		Data: "li",
		Attr: []html.Attribute{
			{Key: "class", Val: "nav-item"},
		},
		FirstChild: &html.Node{
			Type: html.RawNode,
			Data: listItemContent,
		},
	}

	// Append the <li> node to the found node
	node.AppendChild(liNode)

	// Render the modified HTML back to a string
	var buf bytes.Buffer
	w := io.Writer(&buf)
	if err := html.Render(w, doc); err != nil {
		return "", fmt.Errorf("error rendering HTML: %w", err)
	}

	return buf.String(), nil
}

// findPaywalledDiv finds the <div> with class="PAYWALLED" and returns it and its parent.
func findPaywalledDiv(n *html.Node) (*html.Node, *html.Node) {
	if n.Type == html.ElementNode && n.Data == "div" {
		for _, attr := range n.Attr {
			if attr.Key == "class" && attr.Val == "PAYWALLED" {
				return n, n.Parent
			}
		}
	}
	for c := n.FirstChild; c != nil; c = c.NextSibling {
		if result, parent := findPaywalledDiv(c); result != nil {
			return result, parent
		}
	}
	return nil, nil
}

// extractAndReplaceContent processes the HTML content as described.
func extractAndReplaceContent(htmlStr string) (string, *string, error) {
	doc, err := html.Parse(strings.NewReader(htmlStr))
	if err != nil {
		return "", nil, fmt.Errorf("error parsing HTML: %w", err)
	}

	paywalledDiv, parent := findPaywalledDiv(doc)
	if paywalledDiv == nil {
		return htmlStr, nil, nil
		return "", nil, fmt.Errorf("no <div class=\"PAYWALLED\"> found")
	}

	// Collect content after the PAYWALLED div
	var contentAfterDiv bytes.Buffer
	for sibling := paywalledDiv.NextSibling; sibling != nil; sibling = sibling.NextSibling {
		if err := html.Render(&contentAfterDiv, sibling); err != nil {
			return "", nil, fmt.Errorf("error rendering content after div: %w", err)
		}
	}

	// Remove all siblings after the PAYWALLED div
	for sibling := paywalledDiv.NextSibling; sibling != nil; {
		next := sibling.NextSibling
		parent.RemoveChild(sibling)
		sibling = next
	}

	templateContent := `
	{{ if and .LoggedIn .HasPaid }}
		{{ .PaywallTemplate.WalledContent }}
	{{ else if and (.LoggedIn) (not .HasPaid) }}
		{{ .PaywallTemplate.PaywallContent }}
	{{ else }}
		{{ .PaywallTemplate.LoginwallContent }}
	{{ end }}
	`

	// Replace with template content
	templateNode := &html.Node{
		Type: html.RawNode,
		Data: templateContent,
	}
	parent.AppendChild(templateNode)

	// Render the modified HTML back to a string
	var modifiedHTML bytes.Buffer
	if err := html.Render(&modifiedHTML, doc); err != nil {
		return "", nil, fmt.Errorf("error rendering modified HTML: %w", err)
	}

	contentAfterDivString := contentAfterDiv.String()

	return modifiedHTML.String(), &contentAfterDivString, nil
}
