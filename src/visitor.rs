/*!
One-line description.

More detailed description, with

# Example

 */

use crate::{
    error::Error,
    graph::{Cluster, Graph, SubGraph, SubGraphKind},
    style::{Attribute, Attributes, ClusterAttributes, GraphAttributes, Styled},
    Identified, Identifier, RootGraph,
};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AttributeSet {
    Edge,
    EdgeDefaults,
    Node,
    NodeDefaults,
    Graph,
    SubGraphDefaults,
    ClusterGraph,
}

pub trait AttributeWalker {
    fn start(&self, set: AttributeSet) -> Result<(), Error>;

    fn attribute(&self, attribute: &Attribute) -> Result<(), Error>;

    fn end(&self, set: AttributeSet) -> Result<(), Error>;
}

pub trait GraphWalker<A>
where
    A: Attributes,
{
    fn attribute_walker(&self) -> Option<&dyn AttributeWalker> {
        None
    }

    fn node_walker(&self) -> Option<&dyn NodeWalker> {
        None
    }

    fn edge_walker(&self) -> Option<&dyn EdgeWalker> {
        None
    }

    fn sub_graph_walker(&self) -> Option<&dyn SubGraphWalker> {
        None
    }

    fn cluster_graph_walker(&self) -> Option<&dyn ClusterGraphWalker> {
        None
    }

    fn end(&self, id: &Identifier) -> Result<(), Error>;
}

pub trait RootGraphWalker: GraphWalker<GraphAttributes> {
    fn start(&self, id: &Identifier, strict: bool, directed: bool) -> Result<(), Error>;
}

pub trait SubGraphWalker: GraphWalker<GraphAttributes> {
    fn start(&self, id: &Identifier, directed: bool) -> Result<(), Error>;
}

pub trait ClusterGraphWalker: GraphWalker<ClusterAttributes> {
    fn start(&self, id: &Identifier, directed: bool) -> Result<(), Error>;
}

pub trait NodeWalker {
    fn start(&self, id: &Identifier) -> Result<(), Error>;

    fn attribute_walker(&self) -> Option<&dyn AttributeWalker> {
        None
    }

    fn end(&self, id: &Identifier) -> Result<(), Error>;
}

pub trait EdgeWalker {
    fn start(&self, head: &Identifier, tail: &Identifier, directed: bool) -> Result<(), Error>;

    fn attribute_walker(&self) -> Option<&dyn AttributeWalker> {
        None
    }

    fn end(&self, head: &Identifier, tail: &Identifier) -> Result<(), Error>;
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn walk_graph(graph: &RootGraph, walker: &dyn RootGraphWalker) -> Result<(), Error> {
    walker.start(graph.id(), graph.is_strict(), graph.is_directed())?;

    walk_graph_common(graph, walker, false)
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn walk_sub_graph(graph: &SubGraph, walker: &(impl SubGraphWalker + ?Sized)) -> Result<(), Error> {
    walker.start(graph.id(), graph.is_directed())?;

    walk_graph_common(graph, walker, false)
}

fn walk_cluster_graph(
    graph: &Cluster,
    walker: &(impl ClusterGraphWalker + ?Sized),
) -> Result<(), Error> {
    walker.start(graph.id(), graph.is_directed())?;

    walk_graph_common(graph, walker, true)
}

fn walk_graph_common<A>(
    graph: &impl Graph<A>,
    walker: &(impl GraphWalker<A> + ?Sized),
    cluster: bool,
) -> Result<(), Error>
where
    A: Attributes,
{
    if let Some(attributes) = graph.attributes() {
        if let Some(walker) = walker.attribute_walker() {
            walker.start(if cluster {
                AttributeSet::ClusterGraph
            } else {
                AttributeSet::Graph
            })?;
            for attribute in attributes.as_ref().iter() {
                walker.attribute(attribute)?;
            }
            walker.end(if cluster {
                AttributeSet::ClusterGraph
            } else {
                AttributeSet::Graph
            })?;
        }
    }

    if let Some(attributes) = graph.default_graph_attributes() {
        if let Some(walker) = walker.attribute_walker() {
            walker.start(AttributeSet::SubGraphDefaults)?;
            for attribute in attributes.as_ref().iter() {
                walker.attribute(attribute)?;
            }
            walker.end(AttributeSet::SubGraphDefaults)?;
        }
    }

    if let Some(attributes) = graph.default_node_attributes() {
        if let Some(walker) = walker.attribute_walker() {
            walker.start(AttributeSet::NodeDefaults)?;
            for attribute in attributes.as_ref().iter() {
                walker.attribute(attribute)?;
            }
            walker.end(AttributeSet::NodeDefaults)?;
        }
    }

    if let Some(attributes) = graph.default_edge_attributes() {
        if let Some(walker) = walker.attribute_walker() {
            walker.start(AttributeSet::EdgeDefaults)?;
            for attribute in attributes.as_ref().iter() {
                walker.attribute(attribute)?;
            }
            walker.end(AttributeSet::EdgeDefaults)?;
        }
    }

    if let Some(walker) = walker.node_walker() {
        for node in graph.nodes() {
            walker.start(node.id())?;
            if let Some(attributes) = node.attributes() {
                if let Some(walker) = walker.attribute_walker() {
                    walker.start(AttributeSet::EdgeDefaults)?;
                    for attribute in attributes.as_ref().iter() {
                        walker.attribute(attribute)?;
                    }
                    walker.end(AttributeSet::EdgeDefaults)?;
                }
            }
            walker.end(node.id())?;
        }
    }

    if let Some(walker) = walker.edge_walker() {
        for edge in graph.edges() {
            walker.start(edge.head(), edge.tail(), edge.is_directed())?;
            if let Some(attributes) = edge.attributes() {
                if let Some(walker) = walker.attribute_walker() {
                    walker.start(AttributeSet::EdgeDefaults)?;
                    for attribute in attributes.as_ref().iter() {
                        walker.attribute(attribute)?;
                    }
                    walker.end(AttributeSet::EdgeDefaults)?;
                }
            }
            walker.end(edge.head(), edge.tail())?;
        }
    }

    for sub_graph in graph.sub_graphs() {
        match sub_graph {
            SubGraphKind::Graph(g) => {
                if let Some(walker) = walker.sub_graph_walker() {
                    walk_sub_graph(g, walker)?;
                }
            }
            SubGraphKind::Cluster(c) => {
                if let Some(walker) = walker.cluster_graph_walker() {
                    walk_cluster_graph(c, walker)?;
                }
            }
        }
    }

    walker.end(graph.id())
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
