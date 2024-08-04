package paywall

import (
	log "github.com/go-pkgz/lgr"
	"fmt"
)


// new paywall from filepath
func NewPaywall(stringDocs map[string]string, staticContent PaywallStaticContent) (*Paywall, error) {
	
	targetPaywall := newPaywall()

	for path, content := range stringDocs {
		contentWithLoginList, err := addLoginListElement(content)
		if err != nil {
			return nil, fmt.Errorf("error adding login list element path: %s, %v", path, err)
		}
		
		contentExtracted, err := getContentAfterClass(contentWithLoginList, "PAYWALLED")
		if err != nil {
			return nil, fmt.Errorf("error extracting content after class path: %s, %v", path, err)
		}

		contentPaywallReplaced, err := replacePaywallContent(contentWithLoginList)
		if err != nil {
			return nil, fmt.Errorf("error replacing paywall content path: %s, %v", path, err)
		}

		contentLoginScriptAdded, err := appendLoginScript(contentPaywallReplaced, staticContent.LoginScriptGithub)
		if err != nil {
			return nil, fmt.Errorf("error adding login script path: %s, %v", path, err)
		}

		template, err := newPaywallTemplate(path, contentLoginScriptAdded, contentExtracted, staticContent.Registerwall, staticContent.Paywall)

		if err != nil {
			log.Printf("Error creating paywall template path: %s, %v", path, err)
			continue
		}

		targetPaywall.addTemplate(path, *template)
	}

	return targetPaywall, nil
}




func addLoginListElement(htmlString string) (string, error) {
	targetString := `
		{{ if .UserInfo.LoggedIn }}	
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
	{{ if and .UserInfo.LoggedIn .UserInfo.HasPaid }}
		{{ .PaywallContent.WalledContent }}
	{{ else if and (.UserInfo.LoggedIn) (not .UserInfo.HasPaid) }}
		{{ .PaywallContent.PaywallContent }}
	{{ else }}
		{{ .PaywallContent.LoginwallContent }}
	{{ end }}
	`

	htmlStrReplaced, err := replaceContentAfterClass(htmlStr, "PAYWALLED", templateContent)

	if err != nil {
		return "", err
	}

	return htmlStrReplaced, nil
}

func appendLoginScript(htmlStr string, script string) (string, error) {
	result, err := appendHtmlToHtmlNode(htmlStr, script, "body")
	if err != nil {
		return "", err
	}
	return result, nil
}
