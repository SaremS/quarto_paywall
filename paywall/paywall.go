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
func NewPaywall(stringDocs map[string]string, staticContent PaywallStaticContent) *Paywall {
	
	targetPaywall := newPaywall()

	for path, content := range stringDocs {
		contentWithLoginList, err := addLoginListElement(content)
		if err != nil {
			log.Printf("Error adding login list element path: %s, %w", path, err)
			continue
		}
		
		contentExtracted, err := getContentAfterClass(contentWithLoginList, "PAYWALLED")
		if err != nil {
			log.Printf("Error extracting content after class path: %s, %w", path, err)
			continue
		}

		contentPaywallReplaced, err := replacePaywallContent(contentWithLoginList)
		if err != nil {
			log.Printf("Error replacing paywall content path: %s, %w", path, err)
			continue
		}

		contentLoginScriptAdded, err := appendLoginScript(contentPaywallReplaced, staticContent.LoginScriptGithub)




	}
}

func addLoginListElement(htmlString string) (string, error) {
	targetString := `
		{{ if .LoggedIn }}	
			<button class="nav-link" onclick="runLogout()">Logout</button>
		{{ else }}
			<button class="nav-link" onclick="runLoginGithub()">Login</button>
		{{ end }}`

	result, err := appendNewNodeWithContent(htmlString, "navbar-nav navbar-nav-scroll ms-auto", targetString, "li", "class", "nav-item")

	if err != nil {
		return "", err
	}

	return result, nil
}

func replacePaywallContent(htmlStr string) (string, error) {

	templateContent := `
	{{ if and .LoggedIn .HasPaid }}
		{{ .PaywallTemplate.WalledContent }}
	{{ else if and (.LoggedIn) (not .HasPaid) }}
		{{ .PaywallTemplate.PaywallContent }}
	{{ else }}
		{{ .PaywallTemplate.LoginwallContent }}
	{{ end }}
	`

	htmlStrReplaced, err := replaceContentAfterClass(htmlStr, "PAYWALLED", templateContent)

	if err != nil {
		return "", err
	}

	return htmlStrReplaced, nil
}

func appendLoginScript(htmlStr string, script string) (string, error) {
	result, err := appendHtmlToHtmlNode(htmlStr, "body", script)
	if err != nil {
		return "", err
	}
	return result, nil
}
