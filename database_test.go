package main

import (
	"testing"
)

func TestMemoryDatabase(t *testing.T) {
	db := NewMemoryDatabase()
	if db == nil {
		t.Error("NewMemoryDatabase() returned nil")
	}
	
	err := db.Connect()
	if err != nil {
		t.Errorf("Connect() returned error: %v", err)
	}
	
	err = db.Disconnect()
	if err != nil {
		t.Errorf("Disconnect() returned error: %v", err)
	}
	
	err = db.InsertArticle("user", "article")
	if err != nil {
		t.Errorf("InsertArticle() returned error: %v", err)
	}
	
	articles, err := db.FindUser("user")
	if err != nil {
		t.Errorf("FindUser() returned error: %v", err)
	}
	if len(articles) != 1 {
		t.Errorf("FindUser() returned %d articles, expected 1", len(articles))
	}
	if articles[0] != "article" {
		t.Errorf("FindUser() returned article %s, expected 'article'", articles[0])
	}
}
