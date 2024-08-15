package config

import "fmt"

type PaywallConfigElement struct {
	name            string
	path            string
	id              string
	price           int
	currency        string
	cutoffClassname string // last classname before paywall starts
}

func NewConfigElement(name, path, id, price, currency, cutoffClassname string) (*PaywallConfigElement, error) {
	if !isValidPath(path) {
		return nil, fmt.Errorf("invalid path: %v", path)
	}

	priceInt, err := priceStringToMinorInt(price)
	if err != nil {
		return nil, err
	}

	if !isValidCurrency(currency) {
		return nil, fmt.Errorf("invalid currency: %v", currency)
	}

	return &PaywallConfigElement{
		name:            name,
		path:            path,
		id:              id,
		price:           priceInt,
		currency:        currency,
		cutoffClassname: cutoffClassname,
	}, nil
}

func (c *PaywallConfigElement) GetPath() string {
	return c.path
}

func (c *PaywallConfigElement) GetName() string {
	return c.name
}

func (c *PaywallConfigElement) GetId() string {
	return c.id
}

func (c *PaywallConfigElement) GetPrice() int {
	return c.price
}

func (c *PaywallConfigElement) GetCurrency() string {
	return c.currency
}

func (c *PaywallConfigElement) GetCutoffClassname() string {
	return c.cutoffClassname
}
