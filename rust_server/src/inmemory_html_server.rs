use log::{debug, error};
use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;

use html_editor::operation::*;
use html_editor::{parse, Element};

use crate::models::{AuthLevel, SessionStatus};
use crate::utils::{AdvancedDeletable, AdvancedEditable};
use crate::func_utils::extractable_tuples::ExtractableOptionTuple2;

pub struct InMemoryHtml {
    base_dir: String,
    storage_has_paid: HashMap<String, String>,
    storage_has_auth: HashMap<String, String>,
    storage_no_auth: HashMap<String, String>,
}

impl InMemoryHtml {
    pub fn new(base_dir: &str) -> InMemoryHtml {
        let mut storage_has_paid: HashMap<String, String> = HashMap::new();
        let mut storage_has_auth: HashMap<String, String> = HashMap::new();
        let mut storage_no_auth: HashMap<String, String> = HashMap::new();
        for entry in WalkDir::new(base_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("html") {
                let file_path = path.to_string_lossy().to_string();
                match fs::read_to_string(path) {
                    Ok(contents) => {
                        let with_paywall_logic = InMemoryHtml::add_paywall_logic(&contents);
                        storage_has_paid.insert(
                            file_path.clone(),
                            with_paywall_logic
                                .clone()
                                .replace("{{ nav-button-text }}", "User-Area"),
                        );

                        let with_paywall_content_removed = InMemoryHtml::remove_paywalled_content(
                            &(with_paywall_logic.clone()),
                            "./paywall/paywall.html",
                        );
                        storage_has_auth.insert(
                            file_path.clone(),
                            with_paywall_content_removed
                                .replace("{{ nav-button-text }}", "User-Area"),
                        );

                        let with_registerwall_content_removed =
                            InMemoryHtml::remove_paywalled_content(
                                &with_paywall_logic,
                                "./paywall/registerwall.html",
                            );
                        storage_no_auth.insert(
                            file_path,
                            with_registerwall_content_removed
                                .replace("{{ nav-button-text }}", "Login"),
                        );
                    }
                    Err(e) => {
                        error!("Error reading file {:?}: {}", path, e);
                    }
                }
            }
        }
        return InMemoryHtml {
            base_dir: base_dir.to_string(),
            storage_has_paid,
            storage_has_auth,
            storage_no_auth,
        };
    }

    pub async fn get(&self, key: &str, session_status: &SessionStatus) -> Option<String> {
        let full_key = format!("{}/{}", self.base_dir, key);

        debug!("HTML from memory: {}", full_key);

        let result;

        if session_status.auth_level == AuthLevel::NoAuth {
            result = self.storage_no_auth.get(&full_key).map(|x| x.clone());
        } else if session_status.auth_level <= AuthLevel::UserConfirmed {
            result = self.storage_has_auth.get(&full_key).map(|x| x.clone());
        } else {
            result = self.storage_has_paid.get(&full_key).map(|x| x.clone());
        }

        return result;
    }

    fn add_paywall_logic(html: &str) -> String {
        let with_scripts = InMemoryHtml::add_script_links(html);
        let with_modal = InMemoryHtml::add_login_modal(&with_scripts);
        return InMemoryHtml::add_login_logic(&with_modal);
    }

    fn add_script_links(html: &str) -> String {
        let htmx_tag =
            r#"<script src="https://unpkg.com/htmx.org@1.9.8" crossorigin="anonymous"></script>"#;
        let htmxj_tag = r#"<script src="https://unpkg.com/htmx.org/dist/ext/json-enc.js" crossorigin></script>"#;

        let htmx_node = parse(htmx_tag).unwrap()[0].clone();
        let htmxj_node = parse(htmxj_tag).unwrap()[0].clone();

        let mut html_doc = parse(html).unwrap();

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

    fn extract_paywall_data(html: &str) -> Option<(i64, String)> {
        let html_doc = parse(&html).unwrap();

        if let Some(el) = html_doc.query(&Selector::from(".PAYWALLED")) {
            let attrs = &el.attrs;
            let price_option: Option<i64> = attrs 
                .into_iter()
                .find(|x| x.0 == "paywall-price")
                .map(|x| &x.1)
                .map(|x| x.parse().unwrap());
            
            let title_option: Option<String> = attrs
                .into_iter()
                .find(|x| x.0 == "paywall-title")
                .map(|x| (&x.1).to_string());

            let output = (price_option, title_option).extract();

            return output;
        } else {
            return None;
        }
    }

    fn remove_paywalled_content(html: &str, wall_filepath: &str) -> String {
        let mut html_doc = parse(&html).unwrap();

        let selectable = html_doc.query_mut(&Selector::from("main"));

        match selectable {
            Some(el) => {
                if let Some(_) = el.query(&Selector::from(".PAYWALLED")) {
                    el.delete_all_children_after_selector(&Selector::from(".PAYWALLED"));
                    InMemoryHtml::append_paywall_inplace(el, wall_filepath);
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
}
