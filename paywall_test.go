package main

import (
	"strings"
	"testing"
)

func TestAppendListItem(t *testing.T) {
	htmlStr := `<html><head><title>Test</title></head><body><ul class="mylist"><li>Item 1</li><li>Item 2</li></ul></body></html>`
	className := "mylist"
	listItemContent := "somecontent"
	expected := `<html><head><title>Test</title></head><body><ul class="mylist"><li>Item 1</li><li>Item 2</li><li class="nav-item">somecontent</li></ul></body></html>`

	result, err := appendListItem(htmlStr, className, listItemContent)
	if err != nil {
		t.Fatalf("appendListItem() error = %v", err)
	}
	if result != expected {
		t.Errorf("appendListItem() = %v, want %v", result, expected)
	}
}

func TestAddLoginListElement(t *testing.T) {
	htmlStr := `<html><head><title>Test</title></head><body><ul class="navbar-nav navbar-nav-scroll ms-auto"></ul></body></html>`

	expected := `<html><head><title>Test</title></head><body><ul class="navbar-nav navbar-nav-scroll ms-auto"><li class="nav-item">
		{{ if .LoggedIn }}	
			<button class="nav-link" onclick="runLogout()">Logout</button>
		{{ else }}
			<button class="nav-link" onclick="runLogin()">Login</button>
		{{ end }}</li></ul></body></html>`

	removeWhitespace := func(r rune) rune {
		if r == ' ' || r == '\t' || r == '\n' || r == '\r' {
			return -1
		}
		return r
	}

	result, err := addLoginListElement(htmlStr)
	if err != nil {
		t.Fatalf("appendListItem() error = %v", err)
	}
	if strings.Map(removeWhitespace, result) != strings.Map(removeWhitespace, expected) {
		t.Errorf("appendListItem() = %v, want %v", result, expected)
	}
}

func TestAppendScriptTagToHtml(t *testing.T) {
	htmlStr := `<html><head><title>Test</title></head><body></body></html>`
	expected := `<html><head><title>Test</title></head><body><script>function login(prov) {
		  return new Promise((resolve, reject) => {
		    const url = window.location.href + "?close=true";
		    const eurl = encodeURIComponent(url);
		    const win = window.open(
		      "/auth/" + prov + "/login?id=auth-example&from=" + eurl
		    );
		    const interval = setInterval(() => {
		      try {
			if (win.closed) {
			  reject(new Error("Login aborted"));
			  clearInterval(interval);
			  return;
			}
			if (win.location.search.indexOf("error") !== -1) {
			  reject(new Error(win.location.search));
			  win.close();
			  clearInterval(interval);
			  return;
			}
			if (win.location.href.indexOf(url) === 0) {
			  resolve();
			  win.close();
			  clearInterval(interval);
			}
		      } catch (e) {
		      }
		    },100);
		  });
		}

		function runLogin() {
		login("github")
			    .then(() => {
			      window.location.replace(window.location.href);
			    })
		}
		function runLogout() {
		    fetch("/auth/logout")
		      .then(() => {
			window.location.replace(window.location.href);
		      });
	        }</script></body></html>`

	removeWhitespace := func(r rune) rune {
		if r == ' ' || r == '\t' || r == '\n' || r == '\r' {
			return -1
		}
		return r
	}

	result, err := appendScriptTagToHTML(htmlStr)
	if err != nil {
		t.Fatalf("appendListItem() error = %v", err)
	}
	if strings.Map(removeWhitespace, result) != strings.Map(removeWhitespace, expected) {
		t.Errorf("appendListItem() = %v, want %v", result, expected)
	}
}

func TestNewPaywall(t *testing.T) {
	target := `
		<html><head><title>Test</title></head><body><ul class="navbar-nav navbar-nav-scroll ms-auto"></ul></body></html>
	`
	expected := `
	<html><head><title>Test</title></head><body><ul class="navbar-nav navbar-nav-scroll ms-auto"><li class="nav-item">
		{{ if .LoggedIn }}	
			<button class="nav-link" onclick="runLogout()">Logout</button>
		{{ else }}
			<button class="nav-link" onclick="runLogin()">Login</button>
		{{ end }}
		</li></ul>		function login(prov) {
		  return new Promise((resolve, reject) => {
		    const url = window.location.href + "?close=true";
		    const eurl = encodeURIComponent(url);
		    const win = window.open(
		      "/auth/" + prov + "/login?id=auth-example&from=" + eurl
		    );
		    const interval = setInterval(() => {
		      try {
			if (win.closed) {
			  reject(new Error("Login aborted"));
			  clearInterval(interval);
			  return;
			}
			if (win.location.search.indexOf("error") !== -1) {
			  reject(new Error(win.location.search));
			  win.close();
			  clearInterval(interval);
			  return;
			}
			if (win.location.href.indexOf(url) === 0) {
			  resolve();
			  win.close();
			  clearInterval(interval);
			}
		      } catch (e) {
		      }
		    }, 100);
		  });
		}

		function runLogin() {
		login("github")
			    .then(() => {
			      window.location.replace(window.location.href);
			    })
		}
		function runLogout() {
		    fetch("/auth/logout")
		      .then(() => {
			window.location.replace(window.location.href);
		      });
	        }</body></html>`
	loader := SingleTestStringLoader{}

	result := NewPaywall(target, loader)
	if result == nil {
		t.Fatalf("NewPaywall() = %v, want %v", result, expected)
	}
}
