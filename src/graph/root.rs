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
pub struct RootGraph(GraphImpl<GraphAttributes>);

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

impl Default for RootGraph {
    fn default() -> Self {
        let inner: GraphImpl<GraphAttributes> = GraphImpl::anonymous(GraphImplKind::Root(false));
        Self(inner)
    }
}

impl Display for RootGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_strict() {
            write!(f, "strict ")?;
        }
        if self.is_directed() {
            write!(f, "di")?;
        }
        writeln!(f, "graph {} {{", self.id())?;
        display_graph_common(self, f)?;
        writeln!(f, "}}")
    }
}

impl_graph_trait_for!(RootGraph, GraphAttributes);

impl RootGraph {
    pub fn anonymous(strict: bool, directed: bool) -> Self {
        Self(GraphImpl::anonymous(GraphImplKind::Root(strict))).set_directed(directed)
    }

    pub fn new(id: Identifier, strict: bool, directed: bool) -> Self {
        Self(GraphImpl::new(GraphImplKind::Root(strict), id)).set_directed(directed)
    }

    pub fn is_strict(&self) -> bool {
        match self.0.kind {
            GraphImplKind::Root(v) => v,
            _ => unreachable!(),
        }
    }

    pub fn is_directed(&self) -> bool {
        self.0.directed
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
