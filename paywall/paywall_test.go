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
		{{ .PaywallRenderContent.WalledContent }}
	{{ else if and (.UserInfo.LoggedIn) (not .UserInfo.HasPaid) }}
		{{ .PaywallRenderContent.PaywallContent }}
	{{ else }}
		{{ .PaywallRenderContent.LoginwallContent }}
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

func testAppendLoginScript(t *testing.T) {
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
