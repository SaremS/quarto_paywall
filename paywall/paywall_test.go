package paywall

import (
	"testing"
	"strings"
)

func TestAddLoginListElement(t *testing.T) {
	baseHtml := `<html><head></head><body><div class="navbar-nav navbar-nav-scroll ms-auto"></div></body></html>`

	result, err := addLoginListElement(baseHtml)
	if err != nil {
		t.Fatalf("addLoginListElement() error = %v", err)
	}

	target := `<html><head></head><body><div class="navbar-nav navbar-nav-scroll ms-auto"><li class="nav-item">				{{ if .UserInfo.LoggedIn }}	
			<button class="nav-link" onclick="runLogout()">Logout</button>
		{{ else }}
			<button class="nav-link" onclick="runLoginGithub()">Login</button>
		{{ end }}</li></div></body></html>`

	resultReplaced := strings.ReplaceAll(result, " ", "")
	resultReplaced = strings.ReplaceAll(resultReplaced, "\n", "")
	resultReplaced = strings.ReplaceAll(resultReplaced, "\t", "")

	targetReplaced := strings.ReplaceAll(target, " ", "")
	targetReplaced = strings.ReplaceAll(targetReplaced, "\n", "")
	targetReplaced = strings.ReplaceAll(targetReplaced, "\t", "")

	//compare with all whitespace removed
	if resultReplaced != targetReplaced {
		t.Errorf("addLoginListElement() = %v, want %v", resultReplaced, targetReplaced)
	}
}

func TestReplacePaywallContent(t *testing.T) {
	baseHtml := `<html><head></head><body><div class="PAYWALLED"></div><div class="Test">test</div></body></html>`

	result, err := replacePaywallContent(baseHtml)
	if err != nil {
		t.Fatalf("replacePaywallContent() error = %v", err)
	}

	target := `<html><head></head><body><div class="PAYWALLED"></div>	{{ if and .UserInfo.LoggedIn .UserInfo.HasPaid }}
		{{ .PaywallContent.WalledContent }}
	{{ else if and (.UserInfo.LoggedIn) (not .UserInfo.HasPaid) }}
		{{ .PaywallContent.PaywallContent }}
	{{ else }}
		{{ .PaywallContent.LoginwallContent }}
	{{ end }}</body></html>`

	resultReplaced := strings.ReplaceAll(result, " ", "")
	resultReplaced = strings.ReplaceAll(resultReplaced, "\n", "")
	resultReplaced = strings.ReplaceAll(resultReplaced, "\t", "")

	targetReplaced := strings.ReplaceAll(target, " ", "")
	targetReplaced = strings.ReplaceAll(targetReplaced, "\n", "")
	targetReplaced = strings.ReplaceAll(targetReplaced, "\t", "")

	if resultReplaced != targetReplaced {
		t.Errorf("replacePaywallContent() = %v, want %v", resultReplaced, targetReplaced)
	}
}

func TestAppendLoginScript(t *testing.T) {
	baseHtml := `<html><head></head><body></body></html>`
	script := `<script>console.log("test")</script>`

	result, err := appendLoginScript(baseHtml, script)
	if err != nil {
		t.Fatalf("appendLoginScript() error = %v", err)
	}

	target := `<html><head></head><body><script>console.log("test")</script></body></html>`

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

func TestNewPaywallWithPaywalledContent(t *testing.T) {
	stringDocs := map[string]string{
		"test": `<html><head></head><body><div class="navbar-nav navbar-nav-scroll ms-auto"></div><div class="PAYWALLED"></div><div class="Test">test</div></body></html>`,
	}

  	staticContent := PaywallStaticContent{	
		Paywall: `<div>paywall</div>`,
		Registerwall: `<div>registerwall</div>`,
		LoginScriptGithub: `<script>console.log("test")</script>`,
	}

	targetPaywall, err := NewPaywall(stringDocs, staticContent)

	if err != nil {
		t.Fatalf("NewPaywall() error = %v", err)
	}

	target := `<html><head></head><body><div class="navbar-nav navbar-nav-scroll ms-auto"><li class="nav-item"><button class="nav-link" onclick="runLoginGithub()">Login</button></li></div><div class="PAYWALLED"></div><div>registerwall</div><script>console.log("test")</script></body></html>`

	userInfoHasPaid := UserInfoHasPaid{
		UserInfo: UserInfo{
			Name: "",
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
