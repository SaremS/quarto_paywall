package paywall

import (
	"html/template"
	"net/http"
	"strings"
)

type UserInfo struct {
	Name     string
	LoggedIn bool
}

type UserInfoHasPaid struct {
	UserInfo
	HasPaid bool
}

func NewUserInfoHasPaid(name string, loggedIn, hasPaid bool) UserInfoHasPaid {
	return UserInfoHasPaid{
		UserInfo: UserInfo{
			Name:     name,
			LoggedIn: loggedIn,
		},
		HasPaid: hasPaid,
	}
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
	Paywall            string
	Registerwall       string
	LoginScriptGithub  string
	NavbarLoginButton  string
	PaywallContentHtml string
}

type PaywallRenderContent struct {
	UserInfoHasPaid
	PaywallContent
}
