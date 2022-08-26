/*!
One-line description.

More detailed description, with

# Example

 */

use crate::attributes::{EdgeAttributes, GraphAttributes, NodeAttributes, Styled};
use crate::graph::{
    display_graph_common, Edges, Graph, GraphImpl, GraphImplKind, Nodes, SubGraphKind, SubGraphs,
};
use crate::{Edge, Identified, Identifier, Node};
use std::fmt::Display;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct SubGraph(GraphImpl<GraphAttributes>);

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementation Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for SubGraph {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Display for SubGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "subgraph {} {{", self.id())?;
        display_graph_common(self, f)?;
        writeln!(f, "}}")
    }
}

impl_graph_trait_for!(SubGraph, GraphAttributes);

impl SubGraph {
    pub fn anonymous() -> Self {
        Self(GraphImpl::anonymous(GraphImplKind::Graph))
    }

    pub fn new(id: Identifier) -> Self {
        Self(GraphImpl::new(GraphImplKind::Graph, id))
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
