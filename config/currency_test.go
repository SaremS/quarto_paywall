package config

import (
	"testing"
)

func TestCurrencyStringToMinorInt(t *testing.T) {
	amountString := "12.34"

	result, err := currencyStringToMinorInt(amountString)
	if err != nil {
		t.Fatalf("currencyStringToMinorInt() error = %v", err)
	}

	if result != 1234 {
		t.Errorf("currencyStringToMinorInt() = %v, want 1234", result)
	}
}

func TestCurrencyStringToMinorInt_OnlyMajor(t *testing.T) {
	amountString := "12"

	result, err := currencyStringToMinorInt(amountString)
	if err != nil {
		t.Fatalf("currencyStringToMinorInt() error = %v", err)
	}

	if result != 1200 {
		t.Errorf("currencyStringToMinorInt() = %v, want 1234", result)
	}
}

func TestCurrencyStringToMinorInt_SingleMinor(t *testing.T) {
	amountString := "12.3"

	result, err := currencyStringToMinorInt(amountString)
	if err != nil {
		t.Fatalf("currencyStringToMinorInt() error = %v", err)
	}

	if result != 1230 {
		t.Errorf("currencyStringToMinorInt() = %v, want 1234", result)
	}
}
