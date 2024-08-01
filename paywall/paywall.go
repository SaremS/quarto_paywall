package paywall

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
