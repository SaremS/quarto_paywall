package config

import (
	"testing"
)

func TestNewConfigElement(t *testing.T) {
	c, err := newConfigElement(
		"test",
		"/test/whatever.html",
		"abcd",
		"12.34",
		"EUR",
		"PAYWALLED",
	)
	if err != nil {
		t.Errorf("newConfig() error = %v", err)
	}

	if c == nil {
		t.Errorf("NewConfig() = nil, want Config")
	}
}

func TestGetPath(t *testing.T) {
	c, _ := newConfigElement(
		"test",
		"/test/whatever.html",
		"abcd",
		"12.34",
		"EUR",
		"PAYWALLED",
	)
	if c.GetPath() != "/test/whatever.html" {
		t.Errorf("GetPath() = %v, want /test/whatever.html", c.GetPath())
	}
}

func TestGetName(t *testing.T) {
	c, _ := newConfigElement(
		"test",
		"/test/whatever.html",
		"abcd",
		"12.34",
		"EUR",
		"PAYWALLED",
	)
	if c.GetName() != "test" {
		t.Errorf("GetName() = %v, want test", c.GetName())
	}
}

func TestGetId(t *testing.T) {
	c, _ := newConfigElement(
		"test",
		"/test/whatever.html",
		"abcd",
		"12.34",
		"EUR",
		"PAYWALLED",
	)
	if c.GetId() != "abcd" {
		t.Errorf("GetName() = %v, want test", c.GetName())
	}
}

func TestGetPrice(t *testing.T) {
	c, _ := newConfigElement(
		"test",
		"/test/whatever.html",
		"abcd",
		"12.34",
		"EUR",
		"PAYWALLED",
	)
	if c.GetPrice() != 1234 {
		t.Errorf("GetPrice() = %v, want 12.34", c.GetPrice())
	}
}

func TestGetCurrency(t *testing.T) {
	c, _ := newConfigElement(
		"test",
		"/test/whatever.html",
		"abcd",
		"12.34",
		"EUR",
		"PAYWALLED",
	)
	if c.GetCurrency() != "EUR" {
		t.Errorf("GetCurrency() = %v, want EUR", c.GetCurrency())
	}
}

func TestGetCutoffClassname(t *testing.T) {
	c, _ := newConfigElement(
		"test",
		"/test/whatever.html",
		"abcd",
		"12.34",
		"EUR",
		"PAYWALLED",
	)
	if c.GetCutoffClassname() != "PAYWALLED" {
		t.Errorf("GetCutoffClassname() = %v, want PAYWALLED", c.GetCutoffClassname())
	}
}

func TestCurrencyValid(t *testing.T) {
	_, err := newConfigElement(
		"test",
		"/test/whatever.html",
		"abcd",
		"12.34",
		"ASDF",
		"PAYWALLED",
	)
	if err == nil {
		t.Errorf("Invalid Currency should error our")
	}
}
