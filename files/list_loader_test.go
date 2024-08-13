package files

import "testing"

func TestListLoader(t *testing.T) {
	loader := NewListFileLoader(NewDummyFileLoader("test"))
	targetPaths := []string{"test", "test2", "test3"}

	result, err := loader.LoadTargetPaths(targetPaths)
	if err != nil {
		t.Fatalf("loadTargetPaths() error = %v", err)
	}

	if result["test"] != "test" {
		t.Errorf("loadTargetPaths() = %v, want test", result["test"])
	}

	if result["test2"] != "test" {
		t.Errorf("loadTargetPaths() = %v, want test2", result["test2"])
	}

	if result["test3"] != "test" {
		t.Errorf("loadTargetPaths() = %v, want test3", result["test3"])
	}
}
