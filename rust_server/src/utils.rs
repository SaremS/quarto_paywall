use html_editor::{operation::Selector, Element, Node};

pub trait AdvancedEditable {
    /// Insert `node` at `index` in the children all elements that match the `selector`.
    /// If `index` lies outside the range of the children, `target` is inserted as the last
    /// element of children.
    ///
    /// ```
    /// use html_editor::{parse, Node};
    /// use html_editor::operation::*;
    /// use rust_server::utils::AdvancedEditable;
    ///
    /// let html = r#"<div><ul><li>1</li><li>3</li></ul></div>"#;
    ///
    /// let selector = Selector::from("ul");
    /// let html = parse(html)
    ///     .unwrap()
    ///     .insert_at_index_or_push(&selector, Node::new_element(
    ///         "li",
    ///         vec![],
    ///         vec![Node::Text("2".to_string())]
    ///     ), 1)
    ///     .html();
    /// assert_eq!(html, r#"<div><ul><li>1</li><li>2</li><li>3</li></ul></div>"#)
    /// ```
    fn insert_at_index_or_push(
        &mut self,
        selector: &Selector,
        target: Node,
        index: usize,
    ) -> &mut Self;

    /// Insert `node` before (at the same hierarchical level) all (recursive) all child elements
    /// of `selector` that match the `before_selector`.
    /// If `before_selector` is not found, appends to all `selector` nodes.
    ///
    /// ```
    /// use html_editor::{parse, Node};
    /// use html_editor::operation::*;
    /// use rust_server::utils::AdvancedEditable;
    ///
    /// let html = r#"<div><ul><li>1</li><li class="third">3</li></ul></div>"#;
    ///
    /// let selector = Selector::from("ul");
    /// let before_selector = Selector::from("li.third");
    /// let html = parse(html)
    ///     .unwrap()
    ///     .insert_before_selector_or_push(&selector, Node::new_element(
    ///         "li",
    ///         vec![],
    ///         vec![Node::Text("2".to_string())]
    ///     ), &before_selector)
    ///     .html();
    /// assert_eq!(html, r#"<div><ul><li>1</li><li>2</li><li class="third">3</li></ul></div>"#)
    /// ```
    fn insert_before_selector_or_push(
        &mut self,
        selector: &Selector,
        target: Node,
        before_selector: &Selector,
    ) -> &mut Self;
}

pub trait AdvancedDeletable {
    ///Deletes all nodes after `after_selector` at the same level in the hierarchy
    ///
    ///```
    ///use html_editor::{parse, Node};
    ///use html_editor::operation::*;
    ///use rust_server::utils::AdvancedDeletable;
    ///
    ///let html = r#"<div><ul><li>1</li><li class="selectme">2</li><li>3</li></ul></div>"#;
    ///
    ///let selector = Selector::from("li.selectme");
    ///let html = parse(html)
    ///     .unwrap()
    ///     .delete_all_children_after_selector(&selector)
    ///     .html();
    ///assert_eq!(html, r#"<div><ul><li>1</li><li class="selectme">2</li></ul></div>"#)
    ///```
    fn delete_all_children_after_selector(&mut self, after_selector: &Selector) -> &mut Self;

    ///Recursive helper function for `delete_all_children_after_selector`.
    ///Checks if any child contains the `after_selector` somewhere down its subtree
    ///and deletes all nodes, subsequent to that child.
    fn recurse_and_delete_all_after(&mut self, after_selector: &Selector) -> Vec<bool>;
}

impl AdvancedEditable for Element {
    fn insert_at_index_or_push(
        &mut self,
        selector: &Selector,
        target: Node,
        index: usize,
    ) -> &mut Self {
        self.children
            .insert_at_index_or_push(selector, target.clone(), index);
        if selector.matches(self) {
            if index <= self.children.len() {
                self.children.insert(index, target);
            } else {
                self.children.push(target);
            }
        }
        self
    }

    fn insert_before_selector_or_push(
        &mut self,
        selector: &Selector,
        target: Node,
        before_selector: &Selector,
    ) -> &mut Self {
        self.children
            .insert_before_selector_or_push(selector, target.clone(), before_selector);

        if selector.matches(self) {
            let target_position = self.children.iter().position(|x| match x {
                Node::Element(el) => return before_selector.matches(&el),
                _ => return false,
            });
            match target_position {
                Some(index) => self.children.insert(index, target),
                _ => self.children.push(target),
            }
        }
        self
    }
}

impl AdvancedDeletable for Element {
    fn delete_all_children_after_selector(&mut self, after_selector: &Selector) -> &mut Self {
        let _ = AdvancedDeletable::recurse_and_delete_all_after(self, after_selector);

        return self;
    }

    fn recurse_and_delete_all_after(&mut self, after_selector: &Selector) -> Vec<bool> {
        let children_contain_target = self.children.recurse_and_delete_all_after(after_selector);

        if let Some(pos) = children_contain_target.iter().position(|&flag| flag) {
            if pos < self.children.len() - 1 {
                self.children.truncate(pos + 1);
            }
        }

        return vec![children_contain_target.iter().any(|&x| x)];
    }
}

impl AdvancedEditable for Vec<Node> {
    fn insert_at_index_or_push(
        &mut self,
        selector: &Selector,
        target: Node,
        index: usize,
    ) -> &mut Self {
        for node in self.iter_mut() {
            if let Node::Element(el) = node {
                el.children
                    .insert_at_index_or_push(selector, target.clone(), index);
                if selector.matches(&Element {
                    name: el.name.clone(),
                    attrs: el.attrs.clone(),
                    children: vec![],
                }) {
                    if index <= el.children.len() {
                        el.children.insert(index, target.clone());
                    } else {
                        el.children.push(target.clone());
                    }
                }
            }
        }
        self
    }

    fn insert_before_selector_or_push(
        &mut self,
        selector: &Selector,
        target: Node,
        before_selector: &Selector,
    ) -> &mut Self {
        for node in self.iter_mut() {
            if let Node::Element(el) = node {
                el.children.insert_before_selector_or_push(
                    selector,
                    target.clone(),
                    before_selector,
                );
                if selector.matches(&Element {
                    name: el.name.clone(),
                    attrs: el.attrs.clone(),
                    children: vec![],
                }) {
                    let target_position = el.children.iter().position(|x| match x {
                        Node::Element(el2) => return before_selector.matches(&el2),
                        _ => return false,
                    });
                    match target_position {
                        Some(index) => el.children.insert(index, target.clone()),
                        _ => el.children.push(target.clone()),
                    }
                }
            }
        }
        self
    }
}

impl AdvancedDeletable for Vec<Node> {
    fn delete_all_children_after_selector(&mut self, after_selector: &Selector) -> &mut Self {
        let _ = AdvancedDeletable::recurse_and_delete_all_after(self, after_selector);

        return self;
    }

    fn recurse_and_delete_all_after(&mut self, after_selector: &Selector) -> Vec<bool> {
        let mut result: Vec<bool> = Vec::new();

        for node in self.iter_mut() {
            if let Node::Element(el) = node {
                if after_selector.matches(&Element {
                    name: el.name.clone(),
                    attrs: el.attrs.clone(),
                    children: vec![],
                }) {
                    result.push(true);
                } else {
                    result.push(el.recurse_and_delete_all_after(after_selector)[0]);
                }
            } else {
                result.push(false);
            }
        }

        return result;
    }
}

pub enum ResultOrInfo<T, E, I> {
    Ok(T),
    Err(E),
    Info(I),
}
