package config

import (
    "fmt"
    "gopkg.in/yaml.v3"
)



type PaywallConfig struct {
	elements []*PaywallConfigElement
}:

func NewPaywallConfigFromYaml(yaml string) (*PaywallConfig, error) {

}
