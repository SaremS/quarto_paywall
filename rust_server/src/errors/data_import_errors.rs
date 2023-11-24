use log::error;
use thiserror::Error;
use serde::{Serialize, Deserialize};

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum DataImportError {
    #[error("Currency not found: `{0}`")]
    CurrencyNotFoundError(String),
}
