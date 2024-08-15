package paywall

import (
	"gowall/config"
	"net/http/httptest"
	"strings"
	"testing"
)

func TestStripPrefixFromPaths(t *testing.T) {
	p := newPaywall()
	p.addTemplate("test/path", PaywallTemplate{})

	p.StripPrefixFromPaths("test")

	if _, ok := p.GetTemplate("path"); ok {
		t.Errorf("StripPrefixFromPaths() = %v, want %v", ok, false)
	}
}

func TestWriteHtmlReponse(t *testing.T) {
	stringDoc := `<html><head></head><body><div class="navbar-nav navbar-nav-scroll ms-auto"></div><div class="PAYWALLED"></div><div class="Test">test</div></body></html>`

	configElement, err := config.NewConfigElement("test", "test/test", "test", "12.50", "EUR", "PAYWALLED")
	if err != nil {
		t.Fatalf("NewConfigElement() error = %v", err)
	}

	htmlConfigPair := HtmlPaywallConfigPair{
		HtmlString: stringDoc,
		Config:     configElement,
	}

	docsAndConfigs := make(map[string]HtmlPaywallConfigPair)

	docsAndConfigs["test"] = htmlConfigPair

	staticContent := PaywallStaticContent{
		Paywall:           `<div>paywall</div>`,
		Registerwall:      `<div>registerwall</div>`,
		LoginScriptGithub: `<script>console.log("test")</script>`,
	}

	targetPaywall, err := NewPaywallFromStringDocs(docsAndConfigs, staticContent)
	if err != nil {
		t.Fatalf("NewPaywall() error = %v", err)
	}

	userInfoHasPaid := UserInfoHasPaid{
		UserInfo: UserInfo{
			Name:     "",
			LoggedIn: false,
		},
		HasPaid: false,
	}

	rr := httptest.NewRecorder()

	targetPaywall.WriteHtmlReponse(rr, "test", userInfoHasPaid)

	if rr.Code != 200 {
		t.Errorf("WriteHtmlReponse() = %v, want %v", rr.Code, 200)
	}
}

func TestNewPaywallFromStringDocsWithPaywalledContent(t *testing.T) {
	stringDoc := `<html><head></head><body><div class="navbar-nav navbar-nav-scroll ms-auto"></div><div class="PAYWALLED"></div><div class="Test">test</div></body></html>`

	configElement, err := config.NewConfigElement("test", "test/test", "test", "12.50", "EUR", "PAYWALLED")
	if err != nil {
		t.Fatalf("NewConfigElement() error = %v", err)
	}

	htmlConfigPair := HtmlPaywallConfigPair{stringDoc, configElement}

	docsAndConfigs := make(map[string]HtmlPaywallConfigPair)

	docsAndConfigs["test"] = htmlConfigPair

	staticContent := PaywallStaticContent{
		Paywall:           `<div>paywall</div>`,
		Registerwall:      `<div>registerwall</div>`,
		LoginScriptGithub: `<script>console.log("test")</script>`,
		NavbarLoginButton: `test`,
	}

	targetPaywall, err := NewPaywallFromStringDocs(docsAndConfigs, staticContent)
	if err != nil {
		t.Fatalf("NewPaywall() error = %v", err)
	}

	target := `<html><head></head><body><div class="navbar-nav navbar-nav-scroll ms-auto"><li class="nav-item">test</li></div><div class="PAYWALLED"></div><script>console.log("test")</script></body></html>`

	userInfoHasPaid := UserInfoHasPaid{
		UserInfo: UserInfo{
			Name:     "",
			LoggedIn: false,
		},
		HasPaid: false,
	}

	result, err := targetPaywall.GetAsString("test", userInfoHasPaid)
	if err != nil {
		t.Fatalf("GetAsString() error = %v", err)
	}

	resultReplaced := strings.ReplaceAll(result, " ", "")
	resultReplaced = strings.ReplaceAll(resultReplaced, "\n", "")
	resultReplaced = strings.ReplaceAll(resultReplaced, "\t", "")

	targetReplaced := strings.ReplaceAll(target, " ", "")
	targetReplaced = strings.ReplaceAll(targetReplaced, "\n", "")
	targetReplaced = strings.ReplaceAll(targetReplaced, "\t", "")

	if resultReplaced != targetReplaced {
		t.Errorf("appendLoginScript() = %v, want %v", resultReplaced, targetReplaced)
	}
}

func TestPaywallContainsPath_Contained(t *testing.T) {
	p := newPaywall()
	p.addTemplate("test/path", PaywallTemplate{})
	if !p.ContainsPath("test/path") {
		t.Errorf("ContainsPath() = %v, want %v", false, true)
	}
}

func TestPaywallContainsPath_NotContained(t *testing.T) {
	p := newPaywall()
	p.addTemplate("test/path", PaywallTemplate{})
	if !p.ContainsPath("test/path") {
		t.Errorf("ContainsPath() = %v, want %v", false, true)
	}
}
