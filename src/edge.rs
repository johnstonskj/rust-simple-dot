/*!
One-line description.

More detailed description, with

# Example

 */

use crate::{style::EdgeStyle, Node};
use std::fmt::Display;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Edge {
    head: String,
    tail: String,
    directed: bool,
    label: Option<String>,
    style: Option<EdgeStyle>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

display_to_inner!(Edge, true);

impl Edge {
    pub fn new(head: &str, tail: &str) -> Self {
        Self {
            head: head.to_string(),
            tail: tail.to_string(),
            label: Default::default(),
            style: Default::default(),
            directed: Default::default(),
        }
    }

    pub fn new_from(head: &Node, tail: &Node) -> Self {
        Self {
            head: head.id().to_string(),
            tail: tail.id().to_string(),
            label: Default::default(),
            style: Default::default(),
            directed: Default::default(),
        }
    }

    pub fn labeled(head: &str, tail: &str, label: &str) -> Self {
        Self {
            head: head.to_string(),
            tail: tail.to_string(),
            label: Some(label.to_string()),
            style: Default::default(),
            directed: Default::default(),
        }
    }

    pub fn labeled_from(head: &Node, tail: &Node, label: &str) -> Self {
        Self {
            head: head.id().to_string(),
            tail: tail.id().to_string(),
            label: Some(label.to_string()),
            style: Default::default(),
            directed: Default::default(),
        }
    }

    pub fn head(&self) -> &String {
        &self.head
    }

    pub fn set_head(&mut self, head: &str) -> &mut Self {
        self.head = head.to_owned();
        self
    }

    pub fn tail(&self) -> &String {
        &self.tail
    }

    pub fn set_tail(&mut self, tail: &str) -> &mut Self {
        self.tail = tail.to_owned();
        self
    }

    pub fn label(&self) -> Option<&String> {
        self.label.as_ref()
    }

    pub fn set_label(&mut self, label: &str) -> &mut Self {
        self.label = Some(label.to_owned());
        self
    }

    pub fn unset_label(&mut self) -> &mut Self {
        self.label = None;
        self
    }

    pub fn style(&self) -> Option<&EdgeStyle> {
        self.style.as_ref()
    }

    pub fn set_style(&mut self, style: EdgeStyle) -> &mut Self {
        self.style = Some(style);
        self
    }

    pub fn unset_style(&mut self) -> &mut Self {
        self.style = None;
        self
    }

    pub(crate) fn set_directed(mut self, directed: bool) -> Self {
        self.directed = directed;
        self
    }

    pub(crate) fn inner_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        indent_level: u32,
        _in_block: bool,
    ) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.head(),
            if self.directed { "->" } else { "--" },
            self.tail()
        )?;
        if self.label().is_some() || self.style().is_some() {
            write!(f, " [ ")?;
            if let Some(label) = self.label() {
                write!(f, "label = {:?}; ", label)?;
            }
            if let Some(style) = self.style() {
                style.inner_fmt(f, indent_level, false)?;
            }
            writeln!(f, "]")
        } else {
            writeln!(f)
        }
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
    fn test_simple_edges() {
        assert_eq!(Edge::new("a", "b").to_string(), String::from("a -- b\n"));
        assert_eq!(
            Edge::labeled("a", "b", "a to b").to_string(),
            String::from("a -- b [ label = \"a to b\"; ]\n")
        );
        assert_eq!(
            Edge::new("a", "b").set_directed(true).to_string(),
            String::from("a -> b\n")
        );
    }
}
