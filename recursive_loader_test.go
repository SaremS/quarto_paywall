package main

import (
	"testing"
	"html/template"
	"bytes"
)

func TestSingleStringLoader(t *testing.T) {
	loader := SingleTestStringLoader{}
	contentFunc := func(content string) PaywallTemplate {
		tmpl, _ := template.New("test").Parse(content)

		return PaywallTemplate {
			Template: *tmpl,
			WalledContent: nil,
			PaywallContent: nil,
		}
	}
	target := "test"
	fileType := ".txt"
	result, err := loader.WalkTarget(target, fileType, contentFunc)
	if err != nil {
		t.Fatalf("WalkTarget() error = %v", err)
	}
	
	var buf bytes.Buffer

    	// Execute the template, writing to the buffer
    	err = result["test"].Template.Execute(&buf, nil)

	if err != nil {
		t.Fatalf("Error executing template: %v", err)
	}

	outcome := buf.String()

	if outcome != target {
		t.Errorf("WalkTarget() = %v, want %v", *result[target], target)
	}
}
