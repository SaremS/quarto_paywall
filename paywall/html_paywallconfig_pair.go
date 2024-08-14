package paywall

import "gowall/config"

type HtmlPaywallConfigPair struct {
	HtmlString string
	Config     *config.PaywallConfigElement
}
