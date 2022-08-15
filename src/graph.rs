/*!
One-line description.

More detailed description, with

# Example

 */

use crate::style::{EdgeStyle, GraphOptions, GraphStyle, NodeStyle};
use crate::{Edge, Node};
use paste::paste;
use std::collections::HashMap;
use std::fmt::Display;
use unique_id::string::StringGenerator;
use unique_id::Generator;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct RootGraph {
    strict: bool,
    directed: bool,
    inner: Graph,
}

#[derive(Clone, Debug)]
pub struct SubGraph {
    cluster: bool,
    inner: Graph,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
struct Graph {
    id: String,
    label: Option<String>,
    options: Option<GraphOptions>,
    style: Option<GraphStyle>,
    default_node_style: Option<NodeStyle>,
    default_edge_style: Option<EdgeStyle>,
    nodes: HashMap<String, Node>,
    edges: Vec<Edge>,
    sub_graphs: Vec<SubGraph>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Graph {
    fn default() -> Self {
        Self {
            id: StringGenerator::default().next_id(),
            label: Default::default(),
            options: Default::default(),
            style: Default::default(),
            default_node_style: Default::default(),
            default_edge_style: Default::default(),
            nodes: Default::default(),
            edges: Default::default(),
            sub_graphs: Default::default(),
        }
    }
}

impl Graph {
    pub fn new(id: &str) -> Self {
        assert!(!id.is_empty());
        Self {
            id: id.to_owned(),
            ..Default::default()
        }
    }

    pub fn labeled(label: &str) -> Self {
        Self {
            id: StringGenerator::default().next_id(),
            label: Some(label.to_owned()),
            ..Default::default()
        }
    }

    fn anonymous() -> Self {
        Self {
            id: String::new(),
            ..Default::default()
        }
    }

    fn is_anonymous(&self) -> bool {
        self.id.is_empty()
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for RootGraph {
    fn default() -> Self {
        Self {
            strict: Default::default(),
            directed: Default::default(),
            inner: Default::default(),
        }
    }
}

impl From<Graph> for RootGraph {
    fn from(v: Graph) -> Self {
        Self {
            strict: Default::default(),
            directed: Default::default(),
            inner: v,
        }
    }
}

display_to_inner!(RootGraph);

impl AsRef<Graph> for RootGraph {
    fn as_ref(&self) -> &Graph {
        &self.inner
    }
}

impl AsMut<Graph> for RootGraph {
    fn as_mut(&mut self) -> &mut Graph {
        &mut self.inner
    }
}

impl From<RootGraph> for Graph {
    fn from(v: RootGraph) -> Self {
        v.inner
    }
}

impl RootGraph {
    pub fn digraph() -> Self {
        Self {
            strict: Default::default(),
            directed: Default::default(),
            inner: Default::default(),
        }
    }

    pub fn new(id: &str) -> Self {
        Self {
            strict: Default::default(),
            directed: Default::default(),
            inner: Graph::new(id),
        }
    }

    pub fn labeled(label: &str) -> Self {
        Self {
            strict: Default::default(),
            directed: Default::default(),
            inner: Graph::labeled(label),
        }
    }

    pub fn new_directed(id: &str) -> Self {
        Self {
            strict: Default::default(),
            directed: true,
            inner: Graph::new(id),
        }
    }

    pub fn directed_labeled(label: &str) -> Self {
        Self {
            strict: Default::default(),
            directed: true,
            inner: Graph::labeled(label),
        }
    }

    pub fn is_strict(&self) -> bool {
        self.strict
    }

    pub fn set_strict(&mut self, strict: bool) -> &mut Self {
        self.strict = strict;
        self
    }

    pub fn is_directed(&self) -> bool {
        self.directed
    }

    pub(crate) fn set_directed(&mut self, directed: bool) -> &mut Self {
        // TODO: update flag on all owned edges.
        self.directed = directed;
        self
    }

    shared_graph_impl!();

    pub(crate) fn inner_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        indent_level: u32,
        in_block: bool,
    ) -> std::fmt::Result {
        if self.is_strict() {
            write!(f, "strict ")?;
        }
        if self.is_directed() {
            write!(f, "di")?;
        }
        writeln!(f, "graph {} {{", self.id())?;
        self.graph_fmt(f, indent_level + 1, in_block)?;
        write!(f, "}}")
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for SubGraph {
    fn default() -> Self {
        Self {
            cluster: Default::default(),
            inner: Default::default(),
        }
    }
}

impl From<Graph> for SubGraph {
    fn from(v: Graph) -> Self {
        Self {
            cluster: Default::default(),
            inner: v,
        }
    }
}

display_to_inner!(SubGraph);

impl AsRef<Graph> for SubGraph {
    fn as_ref(&self) -> &Graph {
        &self.inner
    }
}

impl AsMut<Graph> for SubGraph {
    fn as_mut(&mut self) -> &mut Graph {
        &mut self.inner
    }
}

impl From<SubGraph> for Graph {
    fn from(v: SubGraph) -> Self {
        v.inner
    }
}

impl SubGraph {
    pub fn cluster() -> Self {
        Self {
            cluster: Default::default(),
            inner: Default::default(),
        }
    }

    pub fn anonymous() -> Self {
        Self {
            cluster: Default::default(),
            inner: Graph::anonymous(),
        }
    }

    pub fn new(id: &str) -> Self {
        Self {
            cluster: Default::default(),
            inner: Graph::new(id),
        }
    }

    pub fn labeled(label: &str) -> Self {
        Self {
            cluster: Default::default(),
            inner: Graph::labeled(label),
        }
    }

    pub fn new_cluster(id: &str) -> Self {
        Self {
            cluster: true,
            inner: Graph::new(id),
        }
    }

    pub fn cluster_labeled(label: &str) -> Self {
        Self {
            cluster: true,
            inner: Graph::labeled(label),
        }
    }

    pub fn is_cluster(&self) -> bool {
        self.cluster
    }

    pub fn set_cluster(&mut self, cluster: bool) -> &mut Self {
        self.cluster = cluster;
        self
    }

    shared_graph_impl!();

    pub(crate) fn inner_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        indent_level: u32,
        in_block: bool,
    ) -> std::fmt::Result {
        writeln!(
            f,
            "subgraph {}{} {{",
            if self.is_cluster() { "cluster_" } else { "" },
            self.id()
        )?;
        self.graph_fmt(f, indent_level + 1, in_block)?;
        writeln!(f, "}}")
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cluster_example() {
        let mut root = RootGraph::new_directed("G");
        let root_graph = root.as_mut();

        let mut cluster = SubGraph::cluster_labeled("process #1");
        let cluster_graph = cluster.as_mut();
        cluster_graph.set_id("0");
        root_graph.add_sub_graph(cluster);

        let mut cluster = SubGraph::cluster_labeled("process #2");
        let cluster_graph = cluster.as_mut();
        cluster_graph.set_id("1");
        root_graph.add_sub_graph(cluster.clone());

        println!("{}", root);
    }
}
