package files

import (
	"testing"
)

func TestSingleStringLoader(t *testing.T) {
	loader := NewDummyPathLoader("test")

	targetPath := "test"
	fileType := ".txt"

	result, err := loader.WalkTarget(targetPath, fileType)

	if err != nil {
		t.Fatalf("WalkTarget() error = %v", err)
	}

	target := "test"
	outcome := *result[targetPath]
	
	if outcome != target {
		t.Errorf("WalkTarget() = %v, want %v", *result[target], target)
	}
}
