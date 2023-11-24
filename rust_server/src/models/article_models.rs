use serde::{Serialize, Deserialize};
use html_editor::{operation::*, parse};

use crate::price::Price;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PaywallArticle {
    identifier: String,
    pub link: String,
    pub title: String,
    price: Price
}

impl PaywallArticle {
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

            return Some(PaywallArticle{ identifier, link: link.to_string(), title, price});
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
