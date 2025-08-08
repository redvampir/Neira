use pyo3::prelude::*;
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;
use once_cell::sync::Lazy;
use regex::Regex;

#[pyclass]
pub struct KnowledgeGraph {
    graph: DiGraph<String, String>,
    nodes: HashMap<String, NodeIndex>,
}

#[pymethods]
impl KnowledgeGraph {
    #[new]
    pub fn new() -> Self {
        KnowledgeGraph { graph: DiGraph::new(), nodes: HashMap::new() }
    }

    pub fn add_fact(&mut self, subject: &str, relation: &str, object: &str) {
        let s_idx = *self.nodes.entry(subject.to_string()).or_insert_with(|| self.graph.add_node(subject.to_string()));
        let o_idx = *self.nodes.entry(object.to_string()).or_insert_with(|| self.graph.add_node(object.to_string()));
        self.graph.add_edge(s_idx, o_idx, relation.to_string());
    }

    pub fn check_claim(&self, claim: &str) -> (Option<bool>, f32) {
        static BELONGS_RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"(?i)^(?P<char>[A-Za-zА-Яа-яЁё]+)\s+принадлежит\s+миру\s+(?P<world>[A-Za-zА-Яа-яЁё]+)$").unwrap()
        });
        static RELATED_RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"(?i)^(?P<src>[A-Za-zА-Яа-яЁё]+)\s+связан\s+с\s+(?P<dst>[A-Za-zА-Яа-яЁё]+)$").unwrap()
        });
        if let Some(caps) = BELONGS_RE.captures(claim) {
            let c = &caps["char"]; let w = &caps["world"]; 
            let exists = self.has_relation(c, w, "belongs_to");
            return (Some(exists), 1.0);
        }
        if let Some(caps) = RELATED_RE.captures(claim) {
            let s = &caps["src"]; let d = &caps["dst"]; 
            let exists = self.has_any_edge(s, d);
            return (Some(exists), 1.0);
        }
        (None, 0.0)
    }
}

impl KnowledgeGraph {
    fn idx(&self, name: &str) -> Option<NodeIndex> {
        self.nodes.get(name).copied()
    }

    fn has_relation(&self, subject: &str, object: &str, relation: &str) -> bool {
        if let (Some(s), Some(o)) = (self.idx(subject), self.idx(object)) {
            self.graph.edges_connecting(s, o).any(|e| e.weight() == relation)
        } else { false }
    }

    fn has_any_edge(&self, a: &str, b: &str) -> bool {
        if let (Some(ai), Some(bi)) = (self.idx(a), self.idx(b)) {
            self.graph.find_edge(ai, bi).is_some() || self.graph.find_edge(bi, ai).is_some()
        } else { false }
    }
}

