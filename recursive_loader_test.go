package main

import (
	"testing"
)

func TestSingleStringLoader(t *testing.T) {
	loader := SingleTestStringLoader{}
	contentFunc := func(content string) string {
		return content
	}
	target := "test"
	fileType := ".txt"
	result, err := loader.WalkTarget(target, fileType, contentFunc)
	if err != nil {
		t.Fatalf("WalkTarget() error = %v", err)
	}
	if *result[target] != target {
		t.Errorf("WalkTarget() = %v, want %v", *result[target], target)
	}
}
