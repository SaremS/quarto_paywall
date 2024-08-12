package config

import (
	"fmt"
	"strconv"
	"strings"
)

func priceStringToMinorInt(amount string) (int, error) {
	parts := strings.SplitN(amount, ".", 2)

	majorPart := 0
	minorPart := 0
	var err error

	majorPart, err = strconv.Atoi(parts[0])
	if err != nil {
		return 0, fmt.Errorf("invalid major part: %v", err)
	}

	majorPart *= 100

	if len(parts) > 1 {
		// Make sure to only take the first two digits of the fractional part
		fractionalPart := parts[1]
		if len(fractionalPart) > 2 {
			fractionalPart = fractionalPart[:2]
		}

		// Pad with zeros if the fractional part has less than 2 digits
		for len(fractionalPart) < 2 {
			fractionalPart += "0"
		}

		minorPart, err = strconv.Atoi(fractionalPart)
		if err != nil {
			return 0, fmt.Errorf("invalid minor part: %v", err)
		}
	}

	// Combine the major and minor parts to get the total in minor units
	totalMinorUnits := majorPart + minorPart

	return totalMinorUnits, nil
}

func isValidCurrency(currency string) bool {
	set := map[string]bool{
		"EUR": true,
		"USD": true,
	}
	return set[currency]
}
