package config

import (
	"encoding/csv"
	"fmt"
	"strings"
)

type PaywallConfig struct {
	elements []*PaywallConfigElement
}

func NewPaywallConfigFromCsvString(csvString string) (*PaywallConfig, error) {
	reader := csv.NewReader(strings.NewReader(csvString))
	reader.FieldsPerRecord = 6
	reader.TrimLeadingSpace = true
	data, err := reader.ReadAll()
	if err != nil {
		return nil, err
	}

	elements := []*PaywallConfigElement{}

	for i, row := range data {
		if i == 0 {
			if !checkHeaderIsValid(row) {
				return nil, fmt.Errorf("invalid header: %v", row)
			}
			continue
		}

		name := row[0]
		path := row[1]
		id := row[2]
		price := row[3]
		currency := row[4]
		cutoffClassname := row[5]
		element, err := newConfigElement(name, path, id, price, currency, cutoffClassname)
		if err != nil {
			return nil, fmt.Errorf("error creating config element for %s: %v", row, err)
		}
		elements = append(elements, element)
	}

	return &PaywallConfig{elements}, nil
}

func (p *PaywallConfig) GetElementAt(index int) (*PaywallConfigElement, error) {
	if index < 0 || index >= len(p.elements) {
		return nil, fmt.Errorf("index out of bounds")
	}
	return p.elements[index], nil
}

func (p *PaywallConfig) GetPathsAsList() []string {
	paths := []string{}
	for _, element := range p.elements {
		paths = append(paths, element.GetPath())
	}
	return paths
}

func checkHeaderIsValid(header []string) bool {
	if header[0] != "name" || header[1] != "path" || header[2] != "id" || header[3] != "price" || header[4] != "currency" || header[5] != "cutoffClassname" {
		return false
	}
	return true
}
