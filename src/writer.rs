/*!
One-line description.

More detailed description, with

# Example

 */

use crate::attributes::{Attributes, Styled};
use crate::error::Error;
use crate::graph::{Cluster, Graph, SubGraph, SubGraphKind};
use crate::{Identified, Node, RootGraph};
use std::cell::RefCell;
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn write_graph<W>(graph: &RootGraph, writer: &mut W) -> Result<(), Error>
where
    W: Write,
{
    let indenter = Indenter::pad_with_two_spaces();
    write_root_graph(graph, writer, &indenter)
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Indenter {
    pad: String,
    stack: RefCell<Vec<String>>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Indenter {
    pub fn pad_with<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            pad: s.into(),
            stack: Default::default(),
        }
    }

    pub fn pad_with_tabs() -> Self {
        Self::pad_with("\t")
    }

    pub fn pad_with_spaces(num: usize) -> Self {
        Self::pad_with(format!("{:width$}", "", width = num))
    }

    pub fn pad_with_two_spaces() -> Self {
        Self::pad_with_spaces(2)
    }

    pub fn pad_with_four_spaces() -> Self {
        Self::pad_with_spaces(4)
    }

    pub fn indent(&self) {
        self.indent_by(1);
    }

    pub fn indent_by(&self, by: usize) {
        (0..by).for_each(|_| {
            self.stack.borrow_mut().push(self.pad.clone());
        });
    }

    pub fn unindent(&self) {
        self.unindent_by(1);
    }

    pub fn unindent_by(&self, by: usize) {
        (0..by).for_each(|_| {
            let _ = self.stack.borrow_mut().pop();
        });
    }

    pub fn level(&self) -> usize {
        self.stack.borrow().len()
    }

    pub fn pad_string(&self) -> String {
        self.stack.borrow().join("")
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn write_root_graph<W>(graph: &RootGraph, w: &mut W, indenter: &Indenter) -> Result<(), Error>
where
    W: Write,
{
    if graph.is_strict() {
        write!(w, "strict ")?;
    }
    if graph.is_directed() {
        write!(w, "di")?;
    }
    writeln!(w, "graph {} {{", graph.id())?;
    indenter.indent();
    write_graph_common(graph, w, indenter)?;
    indenter.unindent();
    writeln!(w, "}}")?;
    Ok(())
}

fn write_sub_graph<W>(graph: &SubGraph, w: &mut W, indenter: &Indenter) -> Result<(), Error>
where
    W: Write,
{
    let pad_string = indenter.pad_string();
    writeln!(w, "{}subgraph {} ", pad_string, graph.id())?;
    indenter.indent();
    write_graph_common(graph, w, indenter)?;
    indenter.unindent();
    writeln!(w, "{}}}", pad_string)?;
    Ok(())
}

fn write_cluster_graph<W>(graph: &Cluster, w: &mut W, indenter: &Indenter) -> Result<(), Error>
where
    W: Write,
{
    let pad_string = indenter.pad_string();
    writeln!(w, "{}subgraph cluster_{} {{", pad_string, graph.id())?;
    indenter.indent();
    write_graph_common(graph, w, indenter)?;
    indenter.unindent();
    writeln!(w, "{}}}", pad_string)?;
    Ok(())
}

fn write_graph_common<A, W>(
    graph: &impl Graph<A>,
    w: &mut W,
    indenter: &Indenter,
) -> Result<(), Error>
where
    A: Attributes,
    W: Write,
{
    let pad_string = indenter.pad_string();

    if graph.has_attributes() {
        if let Some(attributes) = graph.attributes() {
            write_attributes(attributes, w, indenter, false)?;
        }
    }

    if let Some(attributes) = graph.default_graph_attributes() {
        writeln!(w)?;
        write_block_attributes(attributes, w, indenter, "graph")?;
    }

    if let Some(attributes) = graph.default_node_attributes() {
        writeln!(w)?;
        write_block_attributes(attributes, w, indenter, "node")?;
    }

    if let Some(attributes) = graph.default_edge_attributes() {
        writeln!(w)?;
        write_block_attributes(attributes, w, indenter, "edge")?;
    }

    if graph.has_sub_graphs() {
        writeln!(w)?;
        for sub_graph in graph.sub_graphs() {
            match sub_graph {
                SubGraphKind::Graph(g) => write_sub_graph(g, w, indenter)?,
                SubGraphKind::Cluster(c) => write_cluster_graph(c, w, indenter)?,
            }
        }
    }

    let nodes: Vec<&Node> = graph.nodes().filter(|n| n.has_attributes()).collect();
    if !nodes.is_empty() {
        writeln!(w)?;
        for node in graph.nodes().filter(|n| n.has_attributes()) {
            write!(w, "{}{}", pad_string, node.id_and_port())?;
            write_block_attributes(node.attributes().unwrap(), w, indenter, "")?;
        }
    }

    if graph.has_edges() {
        writeln!(w)?;
        for edge in graph.edges() {
            write!(w, "{}{}", pad_string, edge.to_string_no_attributes())?;
            if edge.has_attributes() {
                write_block_attributes(edge.attributes().unwrap(), w, indenter, "")?;
            } else {
                writeln!(w, ";")?;
            }
        }
    }
    Ok(())
}

fn write_attributes<A, W>(
    attributes: &A,
    w: &mut W,
    indenter: &Indenter,
    inline: bool,
) -> Result<(), Error>
where
    A: Attributes,
    W: Write,
{
    let pad_string = indenter.pad_string();
    for attribute in attributes.as_ref().iter() {
        if inline {
            write!(w, "{}; ", attribute)?;
        } else {
            writeln!(w, "{}{};", pad_string, attribute)?;
        }
    }
    Ok(())
}

fn write_block_attributes<A, W>(
    attributes: &A,
    w: &mut W,
    indenter: &Indenter,
    prefix: &str,
) -> Result<(), Error>
where
    A: Attributes,
    W: Write,
{
    let pad_string = indenter.pad_string();
    if !attributes.as_ref().is_empty() {
        if prefix.is_empty() {
            writeln!(w, " [")?;
        } else {
            writeln!(w, "{}{} [", pad_string, prefix)?;
        }
        indenter.indent();
        write_attributes(attributes, w, indenter, false)?;
        indenter.unindent();
        writeln!(w, "{}];", pad_string)?;
    }
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
