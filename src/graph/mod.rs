/*!
One-line description.

More detailed description, with

# Example

 */

use crate::attributes::{Attributes, EdgeAttributes, GraphAttributes, NodeAttributes, Styled};
use crate::{Edge, Identified, Identifier, Node};
use std::fmt::Display;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait Graph<A>: Identified + Styled<A>
where
    A: Attributes,
{
    fn is_directed(&self) -> bool;

    fn default_graph_attributes(&self) -> Option<&GraphAttributes>;
    fn set_default_graph_attributes(self, default_graph_attributes: GraphAttributes) -> Self
    where
        Self: Sized;

    fn default_node_attributes(&self) -> Option<&NodeAttributes>;
    fn set_default_node_attributes(self, default_node_attributes: NodeAttributes) -> Self
    where
        Self: Sized;

    fn default_edge_attributes(&self) -> Option<&EdgeAttributes>;
    fn set_default_edge_attributes(self, default_edge_attributes: EdgeAttributes) -> Self
    where
        Self: Sized;

    fn nodes(&self) -> Nodes<'_>;
    fn has_nodes(&self) -> bool;
    fn add_node(self, node: Node) -> Self
    where
        Self: Sized;
    fn add_nodes(self, nodes: Vec<Node>) -> Self
    where
        Self: Sized;

    fn edges(&self) -> Edges<'_>;
    fn has_edges(&self) -> bool;
    fn add_edge(self, edge: Edge) -> Self
    where
        Self: Sized;
    fn add_edges(self, edges: Vec<Edge>) -> Self
    where
        Self: Sized;

    fn add_edge_between(self, from: Identifier, to: Identifier) -> Self
    where
        Self: Sized,
    {
        self.add_edge(Edge::new(from, to))
    }

    fn sub_graphs(&self) -> SubGraphs<'_>;
    fn has_sub_graphs(&self) -> bool;
    fn add_sub_graph<G>(self, sub_graph: G) -> Self
    where
        G: Into<SubGraphKind>,
        Self: Sized;
    fn add_sub_graphs<G>(self, sub_graphs: Vec<G>) -> Self
    where
        G: Into<SubGraphKind>,
        Self: Sized;

    fn chain(self, nodes: Vec<Node>) -> Self;
    fn circular_chain(self, nodes: Vec<Node>) -> Self;
}

#[derive(Clone, Debug)]
pub enum SubGraphKind {
    Graph(SubGraph),
    Cluster(Cluster),
}

#[derive(Debug)]
pub struct Nodes<'a> {
    iter: std::slice::Iter<'a, Node>,
}

#[derive(Debug)]
pub struct Edges<'a> {
    iter: std::slice::Iter<'a, Edge>,
}

#[derive(Debug)]
pub struct SubGraphs<'a> {
    iter: std::slice::Iter<'a, SubGraphKind>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
struct GraphImpl<A>
where
    A: Attributes,
{
    kind: GraphImplKind,
    directed: bool,
    id: Identifier,
    attributes: Option<A>,
    default_graph_attributes: Option<GraphAttributes>,
    default_node_attributes: Option<NodeAttributes>,
    default_edge_attributes: Option<EdgeAttributes>,
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    sub_graphs: Vec<SubGraphKind>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum GraphImplKind {
    Root(bool),
    Cluster,
    Graph,
}
// ------------------------------------------------------------------------------------------------
// Implementation Macros
// ------------------------------------------------------------------------------------------------

macro_rules! impl_graph_trait_for {
    ($type:ty, $attr_type:ty) => {
        impl Identified for $type {
            fn id(&self) -> &Identifier {
                &self.0.id
            }
        }

        impl Styled<$attr_type> for $type {
            fn attributes(&self) -> Option<&$attr_type> {
                self.0.attributes.as_ref()
            }

            fn set_attributes(self, attributes: $attr_type) -> Self
            where
                Self: Sized,
            {
                let mut self_mut = self;
                self_mut.0.attributes = Some(attributes);
                self_mut
            }
        }

        impl Graph<$attr_type> for $type {
            fn is_directed(&self) -> bool {
                self.0.directed
            }

            fn default_graph_attributes(&self) -> Option<&GraphAttributes> {
                self.0.default_graph_attributes.as_ref()
            }

            fn set_default_graph_attributes(self, default_graph_attributes: GraphAttributes) -> Self
            where
                Self: Sized,
            {
                let mut self_mut = self;
                self_mut.0.default_graph_attributes = Some(default_graph_attributes);
                self_mut
            }

            fn default_node_attributes(&self) -> Option<&NodeAttributes> {
                self.0.default_node_attributes.as_ref()
            }

            fn set_default_node_attributes(self, default_node_attributes: NodeAttributes) -> Self
            where
                Self: Sized,
            {
                let mut self_mut = self;
                self_mut.0.default_node_attributes = Some(default_node_attributes);
                self_mut
            }

            fn default_edge_attributes(&self) -> Option<&EdgeAttributes> {
                self.0.default_edge_attributes.as_ref()
            }

            fn set_default_edge_attributes(self, default_edge_attributes: EdgeAttributes) -> Self
            where
                Self: Sized,
            {
                let mut self_mut = self;
                self_mut.0.default_edge_attributes = Some(default_edge_attributes);
                self_mut
            }

            fn nodes(&self) -> Nodes<'_> {
                Nodes {
                    iter: self.0.nodes.iter(),
                }
            }

            fn has_nodes(&self) -> bool {
                !self.0.nodes.is_empty()
            }

            fn add_node(self, node: Node) -> Self
            where
                Self: Sized,
            {
                let mut self_mut = self;
                self_mut.0.nodes.push(node);
                self_mut
            }

            fn add_nodes(self, nodes: Vec<Node>) -> Self
            where
                Self: Sized,
            {
                let mut self_mut = self;
                self_mut.0.nodes.extend(nodes);
                self_mut
            }

            fn edges(&self) -> Edges<'_> {
                Edges {
                    iter: self.0.edges.iter(),
                }
            }

            fn has_edges(&self) -> bool {
                !self.0.edges.is_empty()
            }

            fn add_edge(self, edge: Edge) -> Self
            where
                Self: Sized,
            {
                let edge = edge.set_directed(self.is_directed());
                let mut self_mut = self;
                self_mut.0.edges.push(edge);
                self_mut
            }

            fn add_edges(self, edges: Vec<Edge>) -> Self
            where
                Self: Sized,
            {
                let directed = self.0.directed;
                println!("dir={};", directed);
                let mut self_mut = self;
                self_mut
                    .0
                    .edges
                    .extend(edges.into_iter().map(|e| e.set_directed(directed)));
                self_mut
            }

            fn sub_graphs(&self) -> SubGraphs<'_> {
                SubGraphs {
                    iter: self.0.sub_graphs.iter(),
                }
            }

            fn has_sub_graphs(&self) -> bool {
                !self.0.sub_graphs.is_empty()
            }

            fn add_sub_graph<G>(self, sub_graph: G) -> Self
            where
                G: Into<SubGraphKind>,
                Self: Sized,
            {
                let sub_graph = sub_graph.into().set_directed(self.is_directed());
                let mut self_mut = self;
                self_mut.0.sub_graphs.push(sub_graph);
                self_mut
            }

            fn add_sub_graphs<G>(self, sub_graphs: Vec<G>) -> Self
            where
                G: Into<SubGraphKind>,
                Self: Sized,
            {
                let directed = self.0.directed;
                let mut self_mut = self;
                self_mut.0.sub_graphs.extend(
                    sub_graphs
                        .into_iter()
                        .map(|s| s.into().set_directed(directed)),
                );
                self_mut
            }

            fn chain(self, nodes: Vec<Node>) -> Self {
                let edges = Edge::chain(&nodes);
                let mut self_mut = self.add_edges(edges);
                self_mut.0.nodes.extend(nodes);
                self_mut
            }

            fn circular_chain(self, nodes: Vec<Node>) -> Self {
                let edges = Edge::circular_chain(&nodes);
                let mut self_mut = self.add_edges(edges);
                self_mut.0.nodes.extend(nodes);
                self_mut
            }
        }

        impl $type {
            pub(crate) fn set_directed(self, directed: bool) -> Self {
                let mut self_mut = self;
                self_mut.0.sub_graphs = self_mut
                    .0
                    .sub_graphs
                    .into_iter()
                    .map(|s| s.set_directed(directed))
                    .collect();
                self_mut.0.edges = self_mut
                    .0
                    .edges
                    .into_iter()
                    .map(|e| e.set_directed(directed))
                    .collect();
                self_mut.0.directed = directed;
                self_mut
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for SubGraphKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Graph(g) => g.to_string(),
                Self::Cluster(c) => c.to_string(),
            }
        )
    }
}

impl From<SubGraph> for SubGraphKind {
    fn from(v: SubGraph) -> Self {
        Self::Graph(v)
    }
}

impl From<Cluster> for SubGraphKind {
    fn from(v: Cluster) -> Self {
        Self::Cluster(v)
    }
}

impl SubGraphKind {
    #[inline]
    pub fn is_sub_graph(&self) -> bool {
        matches!(self, Self::Graph(_))
    }

    #[inline]
    pub fn is_cluster_graph(&self) -> bool {
        matches!(self, Self::Cluster(_))
    }

    pub(crate) fn set_directed(self, directed: bool) -> Self {
        match self {
            Self::Graph(v) => Self::Graph(v.set_directed(directed)),
            Self::Cluster(v) => Self::Cluster(v.set_directed(directed)),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl<'a> Iterator for Nodes<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

// ------------------------------------------------------------------------------------------------

impl<'a> Iterator for Edges<'a> {
    type Item = &'a Edge;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

// ------------------------------------------------------------------------------------------------

impl<'a> Iterator for SubGraphs<'a> {
    type Item = &'a SubGraphKind;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for GraphImplKind {
    fn default() -> Self {
        Self::Graph
    }
}

// ------------------------------------------------------------------------------------------------

impl<A> Default for GraphImpl<A>
where
    A: Attributes,
{
    fn default() -> Self {
        Self {
            kind: Default::default(),
            directed: false,
            id: Identifier::new_graph(),
            attributes: Default::default(),
            default_graph_attributes: Default::default(),
            default_node_attributes: Default::default(),
            default_edge_attributes: Default::default(),
            nodes: Default::default(),
            edges: Default::default(),
            sub_graphs: Default::default(),
        }
    }
}

impl<A> GraphImpl<A>
where
    A: Attributes,
{
    fn anonymous(kind: GraphImplKind) -> Self {
        Self {
            kind,
            ..Default::default()
        }
    }

    fn new(kind: GraphImplKind, id: Identifier) -> Self {
        Self {
            kind,
            id,
            ..Default::default()
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn display_graph_common<A>(
    graph: &impl Graph<A>,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result
where
    A: Attributes,
{
    if graph.has_attributes() {
        if let Some(style) = graph.attributes() {
            writeln!(f, "{}", style)?;
        }
        writeln!(f)?;
    }
    if let Some(default_graph_style) = graph.default_graph_attributes() {
        writeln!(f, "graph [")?;
        writeln!(f, "{}", default_graph_style)?;
        writeln!(f, "]")?;
    }
    if let Some(default_node_style) = graph.default_node_attributes() {
        writeln!(f, "node [")?;
        writeln!(f, "{}", default_node_style)?;
        writeln!(f, "]")?;
    }
    if let Some(default_edge_style) = graph.default_edge_attributes() {
        writeln!(f, "edge [")?;
        writeln!(f, "{}", default_edge_style)?;
        writeln!(f, "]")?;
    }

    if graph.has_nodes() {
        writeln!(
            f,
            "{}",
            graph
                .nodes()
                .map(|n| n.id().to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )?;
        writeln!(f)?;
    }
    for edge in graph.edges() {
        write!(f, "{}", edge)?;
    }
    for sub_graph in graph.sub_graphs() {
        write!(f, "{}", sub_graph)?;
    }
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod root;
pub use root::RootGraph;

pub mod sub_graph;
pub use sub_graph::SubGraph;

pub mod cluster;
pub use cluster::Cluster;
