use html_editor::{operation::*, parse};
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

use crate::price::Price;

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PaywallArticle {
    identifier: String,
    pub link: String,
    pub title: String,
    price: Price,
}

impl PaywallArticle {
    pub fn new(identifier: String, link: String, title: String, price: Price) -> PaywallArticle {
        return PaywallArticle {
            identifier,
            link,
            title,
            price,
        };
    }


    ///same as `.from_html_string` but with `html: String` instead of `html: &str`
    ///This made the paywall logic a lit simpler to implement
    pub fn from_html_string_noref(html: String, link: &str) -> Option<PaywallArticle> {
        let html_doc = parse(&html).unwrap();

        if let Some(el) = html_doc.query(&Selector::from(".PAYWALLED")) {
            let attrs = &el.attrs;

            let identifier = attrs
                .into_iter()
                .find(|x| x.0 == "data-paywall-identifier")
                .map(|x| &x.1)
                .map(|x| x.parse().unwrap())
                .unwrap();

            let title = attrs
                .into_iter()
                .find(|x| x.0 == "data-paywall-title")
                .map(|x| (&x.1).to_string())
                .unwrap();

            let price_in_minor = attrs
                .into_iter()
                .find(|x| x.0 == "data-paywall-price")
                .map(|x| &x.1)
                .map(|x| x.parse().unwrap())
                .unwrap();

            let currency_str = attrs
                .into_iter()
                .find(|x| x.0 == "data-paywall-currency")
                .map(|x| &x.1)
                .unwrap();

            let price = Price::from_currency_string(price_in_minor, currency_str).unwrap();

            return Some(PaywallArticle {
                identifier,
                link: link.to_string(),
                title,
                price,
            });
        } else {
            return None;
        }
    }

    pub fn from_html_string(html: &str, link: &str) -> Option<PaywallArticle> {
        let html_doc = parse(&html).unwrap();

        if let Some(el) = html_doc.query(&Selector::from(".PAYWALLED")) {
            let attrs = &el.attrs;

            let identifier = attrs
                .into_iter()
                .find(|x| x.0 == "data-paywall-identifier")
                .map(|x| &x.1)
                .map(|x| x.parse().unwrap())
                .unwrap();

            let title = attrs
                .into_iter()
                .find(|x| x.0 == "data-paywall-title")
                .map(|x| (&x.1).to_string())
                .unwrap();

            let price_in_minor = attrs
                .into_iter()
                .find(|x| x.0 == "data-paywall-price")
                .map(|x| &x.1)
                .map(|x| x.parse().unwrap())
                .unwrap();

            let currency_str = attrs
                .into_iter()
                .find(|x| x.0 == "data-paywall-currency")
                .map(|x| &x.1)
                .unwrap();

            let price = Price::from_currency_string(price_in_minor, currency_str).unwrap();

            return Some(PaywallArticle {
                identifier,
                link: link.to_string(),
                title,
                price,
            });
        } else {
            return None;
        }
    }

    pub fn link_matches(&self, target: &str) -> bool {
        return self.link == target;
    }

    pub fn identifer_matches(&self, target: &str) -> bool {
        return self.identifier == target;
    }

    pub fn get_price_in_major_unit_str(&self) -> String {
        return self.price.get_in_major_unit_str();
    }

    pub fn get_price_in_minor_unit(&self) -> i64 {
        return self.price.get_in_minor_unit();
    }
}
