package config

type PaywallConfigElement struct {
	path            string
	name            string
	id              string
	price           int
	currency        string
	cutoffClassname string // last classname before paywall starts
}

func newConfigElement(path, name, id, price, currency, cutoffClassname string) (*PaywallConfigElement, error) {
	// Convert the price string to an integer
	priceInt, err := currencyStringToMinorInt(price)
	if err != nil {
		return nil, err
	}

	return &PaywallConfigElement{
		path:            path,
		name:            name,
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
