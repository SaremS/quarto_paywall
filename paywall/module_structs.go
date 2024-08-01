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
