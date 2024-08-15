package paywall

import (
	"gowall/config"
	"testing"
)

func TestNewHtmlPaywallConfigPairFromMap(t *testing.T) {
	htmlMap := make(map[string]string)
	htmlMap["test"] = "value"

	csv := `name, path, id, price, currency, cutoffClassname
  test, test, abcd, 12.34, EUR, PAYWALLED`

	conf, err := config.NewPaywallConfigFromCsvString(csv)
	if err != nil {
		t.Errorf("NewPaywallConfigFromCsvString() error = %v", err)
	}

	pair, err := NewHtmlPaywallConfigPairFromMap(htmlMap, conf)
	if err != nil {
		t.Errorf("Could not create new html config pair from map: error =%v", err)
	}

	if pair["test"].HtmlString != "value" {
		t.Errorf("HtmlString = %v, want value", pair["test"].HtmlString)
	}

	if pair["test"].Config.GetName() != "test" {
		t.Errorf("GetName() = %v, want test", pair["test"].Config.GetName())
	}
}
