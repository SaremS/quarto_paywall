use serde::{Deserialize, Serialize};

use crate::errors::DataImportError;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Price {
    price_in_minor: i64,
    currency: Currency,
}

impl Price {
    pub fn new(price_in_minor: i64, currency: Currency) -> Price {
        return Price {
            price_in_minor,
            currency,
        };
    }

    pub fn from_currency_string(
        price_in_minor: i64,
        currency_str: &str,
    ) -> Result<Price, DataImportError> {
        return Currency::from_string(currency_str)
            .map(|currency| Price::new(price_in_minor, currency));
    }

    pub fn get_in_minor_unit(&self) -> i64 {
        return self.price_in_minor;
    }

    pub fn get_in_major_unit(&self) -> f32 {
        return self.currency.transform_minor_to_major(self.price_in_minor);
    }

    pub fn get_in_major_unit_str(&self) -> String {
        return format!("{:.2}", self.get_in_major_unit());
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
struct Currency {
    currency_code: CurrencyCode,
    major_to_minor_ratio: i64,
}

impl Currency {
    fn new(currency_code: CurrencyCode) -> Currency {
        let major_to_minor_ratio: i64 = match currency_code {
            CurrencyCode::USD => 100,
            CurrencyCode::EUR => 100,
        };
        return Currency {
            currency_code,
            major_to_minor_ratio,
        };
    }

    fn from_string(currency: &str) -> Result<Currency, DataImportError> {
        let currency_code_result = CurrencyCode::from_string(currency);
        let currency = currency_code_result.map(|code| Currency::new(code));

        return currency;
    }

    fn transform_minor_to_major(&self, minor_units: i64) -> f32 {
        return minor_units as f32 / self.major_to_minor_ratio as f32;
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "UPPERCASE")]
enum CurrencyCode {
    USD,
    EUR,
}

impl CurrencyCode {
    fn from_string(currency: &str) -> Result<CurrencyCode, DataImportError> {
        if currency == "USD" {
            return Ok(CurrencyCode::USD);
        } else if currency == "EUR" {
            return Ok(CurrencyCode::EUR);
        } else {
            return Err(DataImportError::CurrencyNotFoundError(currency.to_string()));
        }
    }
}
