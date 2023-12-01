use std::collections::HashMap;

use async_trait::async_trait;

use crate::models::{PaywallArticle, SessionStatus};
use crate::paywall::{ContentAndHash, SessionConditionalObject};

pub enum OptionOrHashMatch<T> {
    Some(T),
    None,
    HashMatch,
}

#[async_trait]
pub trait PaywallServer<T: Clone + Sync + Send, U: SessionConditionalObject<T>> {
    fn new_from_paywall_items(items: Vec<(String, PaywallItem<T, U>)>) -> Self;
    async fn get_content_and_hash(
        &self,
        target: &str,
        session_status: &SessionStatus,
    ) -> Option<ContentAndHash<T>>;
    async fn get_content(&self, target: &str, session_status: &SessionStatus) -> Option<T>;
    async fn get_hash(&self, target: &str, session_status: &SessionStatus) -> Option<String>;
    async fn get_content_if_different_etag(
        &self,
        target: &str,
        session_status: &SessionStatus,
        header_option: Option<&actix_web::http::header::HeaderValue>,
    ) -> OptionOrHashMatch<ContentAndHash<T>> {
        if let Some(content_and_hash) = self.get_content_and_hash(target, session_status).await
        //target exists
        {
            if let Some(header) = header_option {
                //header exists
                if let Ok(target_hash) = header.to_str() {
                    //header can be converted to string
                    if content_and_hash.hash == target_hash {
                        //content hash matches etag hash
                        return OptionOrHashMatch::HashMatch;
                    }
                }
            }
            return OptionOrHashMatch::Some(content_and_hash);
        }
        return OptionOrHashMatch::None;
    }
    async fn has_paywall(&self, target: &str) -> bool;
    async fn get_paywall_article(&self, target: &str) -> Option<PaywallArticle>;
}

#[async_trait]
impl<T: Clone + Sync + Send, U: SessionConditionalObject<T>> PaywallServer<T, U>
    for HashMap<String, PaywallItem<T, U>>
{
    fn new_from_paywall_items(items: Vec<(String, PaywallItem<T, U>)>) -> Self {
        let result: HashMap<String, PaywallItem<T, U>> = items.into_iter().collect();
        return result;
    }

    async fn get_content_and_hash(
        &self,
        target: &str,
        session_status: &SessionStatus,
    ) -> Option<ContentAndHash<T>> {
        let query_option = self.get(target);
        return match query_option {
            Some(object) => Some(object.get_with_hash(session_status).await),
            None => None,
        };
    }

    async fn get_content(&self, target: &str, session_status: &SessionStatus) -> Option<T> {
        let query_option = self.get(target);
        return match query_option {
            Some(object) => Some(object.get(session_status).await),
            None => None,
        };
    }

    async fn get_hash(&self, target: &str, session_status: &SessionStatus) -> Option<String> {
        let query_option = self.get(target);
        return match query_option {
            Some(object) => Some(object.get_hash(session_status).await),
            None => None,
        };
    }

    async fn has_paywall(&self, target: &str) -> bool {
        let query_option = self.get(target);
        return match query_option {
            Some(object) => object.has_paywall().await,
            None => false,
        };
    }

    async fn get_paywall_article(&self, target: &str) -> Option<PaywallArticle> {
        let query_option = self.get(target);
        return match query_option {
            Some(object) => object.get_paywall_article().await,
            None => None,
        };
    }
}

pub struct PaywallItem<T: Clone + Sync + Send, U: SessionConditionalObject<T>> {
    object: U,
    paywall_article: Option<PaywallArticle>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Clone + Sync + Send, U: SessionConditionalObject<T>> PaywallItem<T, U> {
    pub fn new(object: U, paywall_article: Option<PaywallArticle>) -> PaywallItem<T, U> {
        let _marker = std::marker::PhantomData;
        return PaywallItem {
            object,
            paywall_article,
            _marker,
        };
    }
}

impl<T: Clone + Sync + Send, U: SessionConditionalObject<T>> PaywallItem<T, U> {
    async fn get_with_hash(&self, session_status: &SessionStatus) -> ContentAndHash<T> {
        return self.object.get_with_hash(session_status).await;
    }

    async fn get(&self, session_status: &SessionStatus) -> T {
        return self.object.get(session_status).await;
    }

    async fn get_hash(&self, session_status: &SessionStatus) -> String {
        return self.object.get_hash(session_status).await;
    }

    async fn has_paywall(&self) -> bool {
        return match self.paywall_article {
            Some(_) => true,
            None => false,
        };
    }

    async fn get_paywall_article(&self) -> Option<PaywallArticle> {
        return self.paywall_article.clone();
    }
}
