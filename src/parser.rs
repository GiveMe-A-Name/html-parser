use crate::dom::{create_text_element, create_text_node, AttrsMap, Node};

struct HtmlParser {
    pos: usize,
    source: String,
}

impl HtmlParser {
    fn parse_text_node(&mut self) -> Node {
        let text = self.consume_while(|ch| ch != '<');
        create_text_node(text)
    }
}

impl HtmlParser {
    fn parse_element_node(&mut self) -> Node {
        assert!(self.consume_char() == '<');
        let tag_name = self.parse_tag_name();
        let attributes = self.parse_attributes();
        assert!(self.consume_char() == '>');

        let children = self.parse_nodes();
        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '/');
        assert_eq!(self.parse_tag_name(), tag_name);

        assert!(self.consume_char() == '>');

        create_text_element(tag_name, attributes, children)
    }

    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|ch| !ch.is_whitespace() && ch != '>')
    }

    fn parse_attributes(&mut self) -> AttrsMap {
        let mut attributes = AttrsMap::new();
        while self.next_char() != '>' {
            self.consume_whitespace();
            let (k, v) = self.parse_attribute();
            attributes.insert(k, v);
        }
        attributes
    }

    fn parse_attribute(&mut self) -> (String, String) {
        let key = self.parse_key();
        assert!(self.consume_char() == '=');
        let value = self.parse_value();
        (key, value)
    }

    fn parse_key(&mut self) -> String {
        self.consume_while(|ch| ch != '=')
    }

    fn parse_value(&mut self) -> String {
        let quote = self.consume_char();
        assert!(quote == '\'' || quote == '"');
        let value = self.consume_while(|ch| ch != quote);
        assert_eq!(quote, self.consume_char());

        value
    }
}

impl HtmlParser {
    fn parse_nodes(&mut self) -> Vec<Node> {
        let mut nodes = vec![];
        loop {
            self.consume_whitespace();
            if self.eof() || self.start_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        nodes
    }

    fn parse_node(&mut self) -> Node {
        match self.next_char() {
            '<' => self.parse_element_node(),
            _ => self.parse_text_node(),
        }
    }

    fn next_char(&self) -> char {
        assert!(!self.eof());
        self.source.chars().skip(self.pos).next().unwrap()
    }

    fn eof(&self) -> bool {
        self.pos >= self.source.chars().count()
    }

    fn start_with(&self, s: &str) -> bool {
        self.source
            .chars()
            .skip(self.pos)
            .into_iter()
            .collect::<String>()
            .as_str()
            .starts_with(s)
    }

    /// Return the current character, and advance self.pos to the next character.
    fn consume_char(&mut self) -> char {
        let ch = self.next_char();
        self.pos += 1;
        ch
    }

    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(|ch| ch.is_whitespace());
    }
}

/// Parse an HTML document and return the root element.
pub fn parse(source: impl Into<String>) -> Node {
    let mut nodes = HtmlParser {
        pos: 0,
        source: source.into(),
    }
    .parse_nodes();

    // If the document contains a root element, just return it. Otherwise, create one.
    if nodes.len() == 1 {
        nodes.swap_remove(0)
    } else {
        create_text_element("html", AttrsMap::new(), nodes)
    }
}
