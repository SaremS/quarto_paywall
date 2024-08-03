package paywall

import (
	"html/template"
)

type Paywall struct {
	tmpl_map map[string]PaywallTemplate
}

func newPaywall() *Paywall {
	tmpl_map := make(map[string]PaywallTemplate)
	return &Paywall{tmpl_map: tmpl_map}
}

func (p *Paywall) GetTemplate(path string) (*PaywallTemplate, bool) {
	tmpl, ok := p.tmpl_map[path]
	return &tmpl, ok
}

func (p *Paywall) addTemplate(path string, tmpl PaywallTemplate) {
	p.tmpl_map[path] = tmpl
}

type UserInfo struct {
	Name     string
	LoggedIn bool
}

type UserInfoHasPaid {
	Name string

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

func (p *Ren

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
