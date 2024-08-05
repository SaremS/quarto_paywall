package paywall

import (
	"fmt"
	log "github.com/go-pkgz/lgr"
	"strings"
	"net/http"
)

type Paywall struct {
	tmpl_map map[string]PaywallTemplate
}

func newPaywall() *Paywall {
	tmpl_map := make(map[string]PaywallTemplate)
	return &Paywall{tmpl_map: tmpl_map}
}

func (p *Paywall) StripPrefixFromPaths(pathPrefix string) {
	for path, tmpl := range p.tmpl_map {
		if strings.HasPrefix(path, pathPrefix) {
			newPath := strings.TrimPrefix(path, pathPrefix)
			p.tmpl_map[newPath] = tmpl
			delete(p.tmpl_map, path)
		}
	}
}

func (p *Paywall) GetTemplate(path string) (*PaywallTemplate, bool) {
	tmpl, ok := p.tmpl_map[path]
	return &tmpl, ok
}

func (p *Paywall) WriteHtmlReponse(w http.ResponseWriter, path string, userInfoHasPaid UserInfoHasPaid) {
	tmpl, ok := p.GetTemplate(path)
	if !ok {
		http.Error(w, "404 not found", http.StatusNotFound)
		return
	}
	err := tmpl.renderToHttpResponse(w, userInfoHasPaid)
	if err != nil {
		http.Error(w, "500 internal server error", http.StatusInternalServerError)
	}
}

func (p *Paywall) GetAsString(path string, userInfoHasPaid UserInfoHasPaid) (string, error) {
	tmpl, ok := p.GetTemplate(path)
	if !ok {
		return "", nil
	}

	return tmpl.renderToString(userInfoHasPaid)
}

func (p *Paywall) addTemplate(path string, tmpl PaywallTemplate) {
	p.tmpl_map[path] = tmpl
}

// new paywall from filepath
func NewPaywallFromStringDocs(stringDocs map[string]string, staticContent PaywallStaticContent) (*Paywall, error) {

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
