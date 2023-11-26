use serde::{Deserialize, Serialize};
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

use crate::errors::DataImportError;

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Price {
    ///Enables to display a price in both major and minor currency units - 
    ///primarily useful for stripe API which requires minor unit, i.e. Dollar cents
    ///instead of dollars with decimals.
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

    ///Get price from price in minor unit (i64) and a corresponding currency symbol string
    ///(currently supports `USD` and `EUR`).
    /// ```
    ///use rust_server::price::Price;
    ///
    ///let price_in_minor = 100;
    ///let currency_str = "USD";
    ///
    ///let price = Price::from_currency_string(price_in_minor, currency_str).unwrap();
    ///assert_eq!(price.get_in_minor_unit(), 100);
    ///assert_eq!(price.get_in_major_unit_str(), "1.00".to_string());
    /// ```
    pub fn from_currency_string(
        price_in_minor: i64,
        currency_str: &str,
    ) -> Result<Price, DataImportError> {
        return Currency::from_string(currency_str)
            .map(|currency| Price::new(price_in_minor, currency));
    }

    ///Returns the price in the minor unit of the given currency;
    ///e.g. US-cents, Euro-cents (100th of the major currencies USD, EUR). 
    ///```
    ///use rust_server::price::Price;
    ///
    ///let price_in_minor = 100;
    ///let currency_str = "USD";
    ///
    ///let price = Price::from_currency_string(price_in_minor, currency_str).unwrap();
    ///assert_eq!(price.get_in_minor_unit(), 100);
    ///```
    pub fn get_in_minor_unit(&self) -> i64 {
        return self.price_in_minor;
    }

    ///Returns the price in the major unit of the given currency;
    ///e.g. US-Dollar or Euros
    ///```
    ///use rust_server::price::Price;
    ///
    ///let price_in_minor = 100;
    ///let currency_str = "USD";
    ///
    ///let price = Price::from_currency_string(price_in_minor, currency_str).unwrap();
    ///assert_eq!(price.get_in_major_unit(), 1.0);
    ///```
    pub fn get_in_major_unit(&self) -> f32 {
        return self.currency.transform_minor_to_major(self.price_in_minor);
    }

    ///Returns price in the major unit, as a string with two decimals - used
    ///primarily for showing the price in an HTML document.
    ///```
    ///use rust_server::price::Price;
    ///
    ///let price_in_minor = 100;
    ///let currency_str = "USD";
    ///
    ///let price = Price::from_currency_string(price_in_minor, currency_str).unwrap();
    ///assert_eq!(price.get_in_major_unit_str(), "1.00".to_string());
    /// ```
    pub fn get_in_major_unit_str(&self) -> String {
        return format!("{:.2}", self.get_in_major_unit());
    }
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Currency {
    ///Helper struct for dealing with currencies - 
    ///primary application is the Stripe API, which requires the currency
    ///to be presented in minor units, e.g. Dollar cents rather than Dollars. 
    ///
    ///Using the `major_to_minor_ratio` allows us to switch between minor and
    ///major currency units for all other purposes, like displaying the major
    ///unit on an HTML page
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

    ///Create currency frm 
    ///
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
