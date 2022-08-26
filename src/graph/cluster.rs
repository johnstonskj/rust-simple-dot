/*!
One-line description.

More detailed description, with

# Example

 */

use crate::attributes::{
    ClusterAttributes, EdgeAttributes, GraphAttributes, NodeAttributes, Styled,
};
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
pub struct Cluster(GraphImpl<ClusterAttributes>);

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

impl Default for Cluster {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Display for Cluster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "subgraph  cluster_{} {{", self.id())?;
        display_graph_common(self, f)?;
        writeln!(f, "}}")
    }
}

impl_graph_trait_for!(Cluster, ClusterAttributes);

impl Cluster {
    pub fn anonymous() -> Self {
        Self(GraphImpl::anonymous(GraphImplKind::Cluster))
    }

    pub fn new(id: Identifier) -> Self {
        Self(GraphImpl::new(GraphImplKind::Cluster, id))
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
