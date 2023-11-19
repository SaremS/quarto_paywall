use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct PurchaseIntent {
    pub purchase_target: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientReference {
    pub user_id: usize,
    pub target: String,
}
