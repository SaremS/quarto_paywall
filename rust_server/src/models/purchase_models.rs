use serde::{Serialize, Deserialize};
use serde_tuple::{Serialize_tuple, Deserialize_tuple};

use crate::models::PaywallArticle;

#[derive(Clone, Serialize, Deserialize)]
pub struct PurchaseIntent {
    pub purchase_target: String
}

#[derive(Serialize_tuple, Deserialize_tuple, Debug, PartialEq, Eq, Clone)]
pub struct PurchaseReference {
    pub user_id: usize,
    pub article: PaywallArticle,
}
