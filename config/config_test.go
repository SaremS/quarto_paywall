package config

import "testing"

func TestPaywallConfigFromYaml(t *testing.T) {
	yaml := `
  - name: test
    path: /test/whatever
    id: abcd
    price: 12.34
    currency: EUR
    cutoff_classname: PAYWALLED`
}
