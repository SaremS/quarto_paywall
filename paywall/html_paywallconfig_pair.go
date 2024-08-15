package paywall

import "gowall/config"

type HtmlPaywallConfigPair struct {
	HtmlString string
	Config     *config.PaywallConfigElement
}

func NewHtmlPaywallConfigPairFromMap(htmlMap map[string]string, conf *config.PaywallConfig) (map[string]HtmlPaywallConfigPair, error) {
	pairMap := make(map[string]HtmlPaywallConfigPair)
	for path, htmlString := range htmlMap {
		confElement, _ := conf.GetElementAtPath(path)

		pairMap[path] = HtmlPaywallConfigPair{
			HtmlString: htmlString,
			Config:     confElement,
		}
	}
	return pairMap, nil
}
