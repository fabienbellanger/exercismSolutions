use crate::Attrs;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Edge {
    pub node_1: String,
    pub node_2: String,
    pub attrs: HashMap<String, String>,
}

impl Edge {
    pub fn new(a: &str, b: &str) -> Self {
        Self {
            node_1: a.to_string(),
            node_2: b.to_string(),
            attrs: HashMap::new(),
        }
    }
}

impl Attrs for Edge {
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
