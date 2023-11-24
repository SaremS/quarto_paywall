use serde::{Serialize, Deserialize};

use crate::models::PaywallArticle;

#[derive(Clone, Serialize, Deserialize)]
pub struct PurchaseIntent {
    pub purchase_target: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurchaseReference {
    pub user_id: usize,
    pub article: PaywallArticle,
}
