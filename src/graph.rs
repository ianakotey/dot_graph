use crate::{
    node::{Node},
    edge::{Edge},
};
use std::io::prelude::*;
use std::io;

pub struct Graph {
    name: String,
    kind: Kind,
    nodes: Vec<Node>,
    edges: Vec<Edge>
}

impl Graph {
    pub fn new(name: &str, kind: Kind) -> Graph {
        Graph { name: String::from(name), kind: kind, nodes: vec![], edges: vec![] }
    }

    pub fn add_node(&mut self, node: Node) -> () {
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, edge: Edge) -> () {
        self.edges.push(edge);
    }

    pub fn to_dot_string(&self) -> io::Result<String> {
        let mut writer = Vec::new();
        self.render_opts(&mut writer).unwrap();
        let mut s = String::new();
        Read::read_to_string(&mut &*writer, &mut s)?;
        Ok(s)
    }

    /// Renders graph `g` into the writer `w` in DOT syntax.
    /// (Main entry point for the library.)
    fn render_opts<'a,
                    W: Write>
        (&self,
        w: &mut W)
        -> io::Result<()> {
        fn writeln<W: Write>(w: &mut W, arg: &[&str]) -> io::Result<()> {
            for &s in arg {
                w.write_all(s.as_bytes())?;
            }
            write!(w, "\n")
        }

        fn indent<W: Write>(w: &mut W) -> io::Result<()> {
            w.write_all(b"    ")
        }

        let options = &[];

        writeln(w, &[self.kind.keyword(), " ", self.name.as_str(), " {"])?;
        for n in self.nodes.iter() {
            indent(w)?;
            let mut text: Vec<&str> = vec![];
            let node_dot_string: String = n.to_dot_string(options);
            text.push(&node_dot_string.as_str());
            writeln(w, &text)?;
        }

        let edge_symbol = self.kind.edgeop();
        for e in self.edges.iter() {
            indent(w)?;
            let mut text: Vec<&str> = vec![];
            let edge_dot_string: String = e.to_dot_string(edge_symbol);
            text.push(&edge_dot_string.as_str());
            writeln(w, &text)?;
        }

        writeln(w, &["}"])
    }
}

/// Graph kind determines if `digraph` or `graph` is used as keyword
/// for the graph.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Kind {
    Digraph,
    Graph,
}

impl Kind {
    /// The keyword to use to introduce the graph.
    /// Determines which edge syntax must be used, and default style.
    pub fn keyword(&self) -> &'static str {
        match *self {
            Kind::Digraph => "digraph",
            Kind::Graph => "graph"
        }
    }

    /// The edgeop syntax to use for this graph kind.
    pub fn edgeop(&self) -> &'static str {
        match *self {
            Kind::Digraph => "->",
            Kind::Graph => "--",
        }
    }
}