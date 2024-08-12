package config

import (
	"testing"
)

func TestPriceStringToMinorInt(t *testing.T) {
	amountString := "12.34"

	result, err := priceStringToMinorInt(amountString)
	if err != nil {
		t.Fatalf("priceStringToMinorInt() error = %v", err)
	}

	if result != 1234 {
		t.Errorf("priceStringToMinorInt() = %v, want 1234", result)
	}
}

func TestPriceStringToMinorInt_OnlyMajor(t *testing.T) {
	amountString := "12"

	result, err := priceStringToMinorInt(amountString)
	if err != nil {
		t.Fatalf("priceStringToMinorInt() error = %v", err)
	}

	if result != 1200 {
		t.Errorf("priceStringToMinorInt() = %v, want 1234", result)
	}
}

func TestPriceStringToMinorInt_SingleMinor(t *testing.T) {
	amountString := "12.3"

	result, err := priceStringToMinorInt(amountString)
	if err != nil {
		t.Fatalf("priceStringToMinorInt() error = %v", err)
	}

	if result != 1230 {
		t.Errorf("priceStringToMinorInt() = %v, want 1234", result)
	}
}
