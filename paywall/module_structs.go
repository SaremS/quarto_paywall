package paywall

import (
	"html/template"
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

type UserInfo struct {
	Name     string
	LoggedIn bool
}

type UserInfoHasPaid struct {
	UserInfo
	HasPaid bool
}

type PaywallTemplate struct {
	Template template.Template
	Content  PaywallContent
}

func newPaywallTemplate(path, content, walledContent, loginwallContent, paywallContent string) (*PaywallTemplate, error) {
	tmpl, err := template.New(path).Parse(content)
	if err != nil {
		return nil, err
	}
	return &PaywallTemplate{
		Template: *tmpl,
		Content:  newPaywallContent(walledContent, loginwallContent, paywallContent),
	}, nil
}

func (p *PaywallTemplate) renderToHttpResponse(w http.ResponseWriter, userInfoHasPaid UserInfoHasPaid) error {
	return p.Template.Execute(w, PaywallRenderContent{
		UserInfoHasPaid: userInfoHasPaid,
		PaywallContent:  p.Content,
	})	
}

func (p *PaywallTemplate) renderToString(userInfoHasPaid UserInfoHasPaid) (string, error) {
	var buf strings.Builder
	err := p.Template.Execute(&buf, PaywallRenderContent{
		UserInfoHasPaid: userInfoHasPaid,
		PaywallContent:  p.Content,
	})
	if err != nil {
		return "", err
	}
	return buf.String(), nil
}

type PaywallContent struct {
	WalledContent    template.HTML 
	LoginwallContent template.HTML
	PaywallContent   template.HTML
}

func newPaywallContent(walledContent, loginwallContent, paywallContent string) PaywallContent {
	return PaywallContent{
		WalledContent:    template.HTML(walledContent),
		LoginwallContent: template.HTML(loginwallContent),
		PaywallContent:   template.HTML(paywallContent),
	}
}

type PaywallStaticContent struct {
	Paywall           string
	Registerwall      string
	LoginScriptGithub string
}

type PaywallRenderContent struct {
	UserInfoHasPaid
	PaywallContent
}
