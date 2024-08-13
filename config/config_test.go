package config

import "testing"

func TestNewPaywallConfigFromCsvString(t *testing.T) {
	csv := `name, path, id, price, currency, cutoffClassname
  test, /test/whatever, abcd, 12.34, EUR, PAYWALLED`

	config, err := NewPaywallConfigFromCsvString(csv)
	if err != nil {
		t.Errorf("NewPaywallConfigFromCsvString() error = %v", err)
	}

	element, err := config.GetElementAt(0)
	if err != nil {
		t.Errorf("GetElementAt() error = %v", err)
	}
	if element.GetName() != "test" {
		t.Errorf("GetName() = %v, want test", element.GetName())
	}

	if element.GetPath() != "/test/whatever" {
		t.Errorf("GetPath() = %v, want /test/whatever", element.GetPath())
	}

	if element.GetId() != "abcd" {
		t.Errorf("GetId() = %v, want abcd", element.GetId())
	}

	if element.GetPrice() != 1234 {
		t.Errorf("GetPrice() = %v, want 1234", element.GetPrice())
	}

	if element.GetCurrency() != "EUR" {
		t.Errorf("GetCurrency() = %v, want EUR", element.GetCurrency())
	}

	if element.GetCutoffClassname() != "PAYWALLED" {
		t.Errorf("GetCutoffClassname() = %v, want PAYWALLED", element.GetCutoffClassname())
	}
}

func TestGetPathsAsList(t *testing.T) {
	csv := `name, path, id, price, currency, cutoffClassname
  test, /test/whatever, abcd, 12.34, EUR, PAYWALLED
  test2, /test/whatever2, abcd2, 12.34, EUR, PAYWALLED`
	config, err := NewPaywallConfigFromCsvString(csv)
	if err != nil {
		t.Errorf("NewPaywallConfigFromCsvString() error = %v", err)
	}
	paths := config.GetPathsAsList()
	if len(paths) != 2 {
		t.Errorf("GetPathsAsList() = %v, want 2", len(paths))
	}
	if paths[0] != "/test/whatever" {
		t.Errorf("GetPathsAsList() = %v, want /test/whatever", paths[0])
	}
	if paths[1] != "/test/whatever2" {
		t.Errorf("GetPathsAsList() = %v, want /test/whatever2", paths[1])
	}
}
