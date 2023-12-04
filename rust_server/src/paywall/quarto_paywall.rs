use std::fs;

use html_editor::{operation::*, parse, Element};
use md5;

use crate::models::{AuthLevel, PaywallArticle};
use crate::paywall::{
    paywall_server_factory, AuthLevelConditionalObject, AuthLevelManipulatorByFn, PaywallServer,
    RecursiveFileReaderString,
};
use crate::price::Price;
use crate::utils::{AdvancedDeletable, AdvancedEditable};


pub fn make_quarto_paywall<V: PaywallServer<String, AuthLevelConditionalObject<String>>>(
    base_dir: &str,
) -> V {
    let filereader = RecursiveFileReaderString::new(base_dir, vec!["html"]);

    let manipulation: Vec<(AuthLevel, fn(String) -> String)> = vec![
        (AuthLevel::NoAuth, noauth_manipulation),
        (AuthLevel::UserUnconfirmed, userunconfirmed_manipulation),
        (AuthLevel::UserConfirmed, userconfirmed_manipulation),
        (AuthLevel::PaidAuth, paidauth_manipulation),
    ]; //compiler throws an error if we remove the type hint
    let manipulator: AuthLevelManipulatorByFn<String, String> =
        AuthLevelManipulatorByFn::new(manipulation, |x| format!("{:x}", md5::compute(x)));

    let paywall_extraction = PaywallArticle::from_html_string_noref;

    let paywall = paywall_server_factory::<String, String, AuthLevelConditionalObject<String>, V>(
        filereader,
        manipulator,
        paywall_extraction,
    );

    return paywall;
}

//TODO: Filetypes can likely be optimized - either use &str or html_editor::parse output
//
trait PaywallExtraction {
    fn is_paywalled(&self) -> bool;
    fn get_paywall_price(&self) -> Price;
}

impl PaywallExtraction for String {
    fn is_paywalled(&self) -> bool {
        return self.contains("class=\"PAYWALLED\"");
    }

    fn get_paywall_price(&self) -> Price {
        let html_doc = parse(self).unwrap();
        let paywall_div = html_doc.query(&Selector::from(".PAYWALLED")).unwrap();

        let attr = &paywall_div.attrs;
        let price_in_minor = attr
            .into_iter()
            .find(|x| x.0 == "data-paywall-price")
            .map(|x| &x.1)
            .map(|x| x.parse().unwrap())
            .unwrap();

        let currency_str = attr
            .into_iter()
            .find(|x| x.0 == "data-paywall-currency")
            .map(|x| &x.1)
            .unwrap();

        let price = Price::from_currency_string(price_in_minor, currency_str).unwrap();

        return price;
    }
}

fn noauth_manipulation(x: String) -> String {
    let with_paidauth =
        paidauth_manipulation_nonav(x.clone()).replace("{{ nav-button-text }}", "Login");

    if x.is_paywalled() {
        return remove_paywalled_content(&with_paidauth, "./paywall/registerwall.html");
    } else {
        return with_paidauth.to_string();
    }
}

fn userunconfirmed_manipulation(x: String) -> String {
    let with_paidauth =
        paidauth_manipulation_nonav(x.clone()).replace("{{ nav-button-text }}", "User-Area");
    if x.is_paywalled() {
        return remove_paywalled_content(&with_paidauth, "./paywall/verifywall.html");
    } else {
        return with_paidauth.to_string();
    }
}

fn userconfirmed_manipulation(x: String) -> String {
    let with_paidauth =
        paidauth_manipulation_nonav(x.clone()).replace("{{ nav-button-text }}", "User-Area");
    if x.is_paywalled() {
        let with_paywall = remove_paywalled_content(&with_paidauth, "./paywall/paywall.html");
        let price = with_paywall.get_paywall_price();
        let with_price =
            with_paywall.replace("{{ paywall-price }}", &price.get_in_major_unit_str());

        return with_price;
    } else {
        return with_paidauth.to_string();
    }
}

fn paidauth_manipulation(x: String) -> String {
    let with_paidauth =
        paidauth_manipulation_nonav(x.clone()).replace("{{ nav-button-text }}", "User-Area");

    return with_paidauth;
}

fn paidauth_manipulation_nonav(x: String) -> String {
    let with_scripts = add_script_links(x);
    let with_modal = add_login_modal(&with_scripts);
    return add_login_logic(&with_modal);
}

fn add_script_links(html: String) -> String {
    let htmx_tag =
        r#"<script src="https://unpkg.com/htmx.org@1.9.8" crossorigin="anonymous"></script>"#;
    let htmxj_tag =
        r#"<script src="https://unpkg.com/htmx.org/dist/ext/json-enc.js" crossorigin></script>"#;

    let htmx_node = parse(htmx_tag).unwrap()[0].clone();
    let htmxj_node = parse(htmxj_tag).unwrap()[0].clone();

    let mut html_doc = parse(&html).unwrap();

    let result = html_doc
        .insert_to(&Selector::from("head"), htmx_node)
        .insert_to(&Selector::from("head"), htmxj_node)
        .trim()
        .html();

    return String::from(result);
}

fn add_login_logic(html: &str) -> String {
    let login_button_html = fs::read_to_string("./paywall/login_button_html.html").unwrap();
    let login_button_script = fs::read_to_string("./paywall/login_button_script.html").unwrap();

    let button_node = parse(&login_button_html).unwrap()[0].clone();
    let script_node = parse(&login_button_script).unwrap()[0].clone();
    let mut html_doc = parse(&html).unwrap();

    let result = html_doc
        .insert_before_selector_or_push(
            &Selector::from("ul.navbar-nav"),
            button_node,
            &Selector::from("li.nav-item.compact"),
        )
        .insert_to(&Selector::from("body"), script_node)
        .trim()
        .html();

    return String::from(result);
}

fn add_login_modal(html: &str) -> String {
    let login_modal_html = fs::read_to_string("./paywall/login_modal_html.html").unwrap();
    let login_modal_style = fs::read_to_string("./paywall/login_modal_style.html").unwrap();

    let modal_node = parse(&login_modal_html).unwrap()[0].clone();
    let style_node = parse(&login_modal_style).unwrap()[0].clone();
    let mut html_doc = parse(&html).unwrap();

    let result = html_doc
        .insert_to(&Selector::from("body"), modal_node)
        .insert_to(&Selector::from("head"), style_node)
        .trim()
        .html();

    return String::from(result);
}

fn remove_paywalled_content(html: &str, wall_filepath: &str) -> String {
    let mut html_doc = parse(&html).unwrap();

    let selectable = html_doc.query_mut(&Selector::from("main"));

    match selectable {
        Some(el) => {
            if let Some(_) = el.query(&Selector::from(".PAYWALLED")) {
                el.delete_all_children_after_selector(&Selector::from(".PAYWALLED"));
                append_paywall_inplace(el, wall_filepath);
            }
            return String::from(html_doc.html());
        }
        None => {
            return String::from(html_doc.html());
        }
    }
}

fn append_paywall_inplace(html_doc: &mut Element, wall_filepath: &str) {
    let paywall_html = fs::read_to_string(wall_filepath).unwrap();
    let paywall_node = parse(&paywall_html).unwrap()[0].clone();

    html_doc
        .insert_to(&Selector::from("main"), paywall_node)
        .trim();
}
