use crate::{edge::Edge, node::Node};
use std::collections::HashMap;

pub mod graph_items {
    pub mod node {
        pub use crate::node::Node;
    }
    pub mod edge {
        pub use crate::edge::Edge;
    }
}

#[derive(Debug, Clone)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: HashMap<String, String>,
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            edges: vec![],
            attrs: HashMap::new(),
        }
    }

    pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
        self.nodes = nodes.to_vec();
        self
    }

    pub fn with_edges(mut self, edges: &[Edge]) -> Self {
        self.edges = edges.to_vec();
        self
    }

    pub fn node(&self, name: &str) -> Option<&Node> {
        self.nodes.iter().find(|node| node.name == name)
    }

    pub fn with_attrs(mut self, attributes: &[(&str, &str)]) -> Self {
        self.attrs = attributes
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect();
        self
    }

    pub fn get_attr(&self, key: &str) -> Option<&str> {
        self.attrs.get(key).map(|s| s.as_str())
    }
}
