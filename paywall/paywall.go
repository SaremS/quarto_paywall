package paywall

import (
	log "github.com/go-pkgz/lgr"
)


// new paywall from filepath
func NewPaywall(stringDocs map[string]string, staticContent PaywallStaticContent) *Paywall {
	
	targetPaywall := newPaywall()

	for path, content := range stringDocs {
		contentWithLoginList, err := addLoginListElement(content)
		if err != nil {
			log.Printf("Error adding login list element path: %s, %v", path, err)
			continue
		}
		
		contentExtracted, err := getContentAfterClass(contentWithLoginList, "PAYWALLED")
		if err != nil {
			log.Printf("Error extracting content after class path: %s, %v", path, err)
			continue
		}

		contentPaywallReplaced, err := replacePaywallContent(contentWithLoginList)
		if err != nil {
			log.Printf("Error replacing paywall content path: %s, %v", path, err)
			continue
		}

		contentLoginScriptAdded, err := appendLoginScript(contentPaywallReplaced, staticContent.LoginScriptGithub)
		if err != nil {
			log.Printf("Error adding login script path: %s, %v", path, err)
			continue
		}

		template, err := newPaywallTemplate(path, contentLoginScriptAdded, contentExtracted, staticContent.Registerwall, staticContent.Paywall)

		if err != nil {
			log.Printf("Error creating paywall template path: %s, %v", path, err)
			continue
		}

		targetPaywall.addTemplate(path, *template)
	}

	return targetPaywall
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
		{{ .PaywallRenderContent.WalledContent }}
	{{ else if and (.UserInfo.LoggedIn) (not .UserInfo.HasPaid) }}
		{{ .PaywallRenderContent.PaywallContent }}
	{{ else }}
		{{ .PaywallRenderContent.LoginwallContent }}
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
