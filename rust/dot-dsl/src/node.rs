use crate::Attrs;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub name: String,
    pub attrs: HashMap<String, String>,
}

impl Node {
    pub fn new(s: &str) -> Self {
        Self {
            name: s.to_string(),
            attrs: HashMap::new(),
        }
    }

    pub fn get_attr(&self, key: &str) -> Option<&str> {
        self.attrs.get(key).map(|s| s.as_str())
    }
}

impl Attrs for Node {
    fn with_attrs(mut self, attributes: &[(&str, &str)]) -> Self {
        self.attrs = attributes
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect();
        self
    }

    fn get_attr(&self, key: &str) -> Option<&str> {
        self.attrs.get(key).map(|s| s.as_str())
    }
}
