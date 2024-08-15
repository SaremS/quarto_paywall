package paywall

import (
	"fmt"
	"gowall/config"
	"net/http"
	"strings"

	log "github.com/go-pkgz/lgr"
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
		log.Printf("404 not found path: %s", path)
		http.Error(w, "404 not found", http.StatusNotFound)
		return
	}
	err := tmpl.renderToHttpResponse(w, userInfoHasPaid)
	if err != nil {
		log.Fatalf("Error executing template on path %s: %v", path, err)
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

func (p *Paywall) ContainsPath(path string) bool {
	_, ok := p.tmpl_map[path]
	return ok
}

func (p *Paywall) addTemplate(path string, tmpl PaywallTemplate) {
	p.tmpl_map[path] = tmpl
}

// new paywall from filepath
func NewPaywallFromStringDocs(docsAndConfigs map[string]HtmlPaywallConfigPair, staticContent PaywallStaticContent) (*Paywall, error) {
	targetPaywall := newPaywall()

	for path, docAndConfig := range docsAndConfigs {
		content := docAndConfig.HtmlString
		contentWithLoginList, err := appendNewNodeWithContent(content, "navbar-nav navbar-nav-scroll ms-auto", staticContent.NavbarLoginButton, "li", "class", "nav-item")
		if err != nil {
			return nil, fmt.Errorf("error adding login list element path: %s, %v", path, err)
		}

		conf := docAndConfig.Config
		if conf == nil {
			template, err := newPaywallTemplate(path, contentWithLoginList, "", staticContent.Registerwall, staticContent.Paywall)
			if err != nil {
				return nil, fmt.Errorf("error creating paywall template path: %s, %v", path, err)
			}

			targetPaywall.addTemplate(path, *template)

		} else {
			template, err := createTemplateForPaywalledSite(contentWithLoginList, path, conf, staticContent)
			if err != nil {
				return nil, fmt.Errorf("error creating paywall template path: %s, %v", path, err)
			}
			targetPaywall.addTemplate(path, *template)
		}
	}

	return targetPaywall, nil
}

func createTemplateForPaywalledSite(content, path string, conf *config.PaywallConfigElement, staticContent PaywallStaticContent) (*PaywallTemplate, error) {
	contentExtracted, err := getContentAfterClass(content, conf.GetCutoffClassname())
	if err != nil {
		return nil, fmt.Errorf("error extracting content after class path: %s, %v", path, err)
	}

	contentPaywallReplaced, err := replaceContentAfterClass(content, conf.GetCutoffClassname(), staticContent.PaywallContentHtml)
	if err != nil {
		return nil, fmt.Errorf("error replacing paywall content path: %s, %v", path, err)
	}

	contentLoginScriptAdded, err := appendHtmlToHtmlNode(contentPaywallReplaced, staticContent.LoginScriptGithub, "body")
	if err != nil {
		return nil, fmt.Errorf("error adding login script path: %s, %v", path, err)
	}

	template, err := newPaywallTemplate(path, contentLoginScriptAdded, contentExtracted, staticContent.Registerwall, staticContent.Paywall)
	if err != nil {
		return nil, fmt.Errorf("error creating paywall template path: %s, %v", path, err)
	}

	return template, nil
}
