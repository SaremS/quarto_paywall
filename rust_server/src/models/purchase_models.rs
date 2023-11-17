use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct PurchaseIntent {
    pub purchase_target: String
}
