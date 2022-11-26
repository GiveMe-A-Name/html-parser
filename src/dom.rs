use std::collections::HashMap;

pub enum Node {
    Element(Element),
    Text(String),
}

pub type AttrsMap = HashMap<String, String>;

pub struct Element {
    pub tag: String,
    pub attributes: AttrsMap,
    pub children: Vec<Node>,
}

pub fn create_text_node(text: impl Into<String>) -> Node {
    Node::Text(text.into())
}

pub fn create_text_element(
    tag: impl Into<String>,
    attributes: AttrsMap,
    children: Vec<Node>,
) -> Node {
    Node::Element(Element {
        tag: tag.into(),
        attributes,
        children,
    })
}
