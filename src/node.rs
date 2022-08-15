/*!
One-line description.

More detailed description, with

# Example

 */

use crate::style::NodeStyle;
use std::fmt::Display;
use unique_id::{string::StringGenerator, Generator};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Node {
    id: String,
    label: Option<String>,
    style: Option<NodeStyle>,
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

impl Default for Node {
    fn default() -> Self {
        Self {
            id: StringGenerator::default().next_id(),
            label: Default::default(),
            style: Default::default(),
        }
    }
}

display_to_inner!(Node, true);

impl Node {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            ..Default::default()
        }
    }

    pub fn labeled(label: &str) -> Self {
        Self {
            id: StringGenerator::default().next_id(),
            label: Some(label.to_string()),
            ..Default::default()
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn set_id(&mut self, id: &str) -> &mut Self {
        self.id = id.to_owned();
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

    pub fn style(&self) -> Option<&NodeStyle> {
        self.style.as_ref()
    }

    pub fn set_style(&mut self, style: NodeStyle) -> &mut Self {
        self.style = Some(style);
        self
    }

    pub fn unset_style(&mut self) -> &mut Self {
        self.style = None;
        self
    }

    pub(crate) fn inner_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        indent_level: u32,
        _in_block: bool,
    ) -> std::fmt::Result {
        write!(f, "{}", self.id)?;
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
    fn test_simple_nodes() {
        assert_eq!(Node::new("a").to_string(), String::from("a\n"));
        assert_eq!(
            Node::new("a").set_label("An A").to_string(),
            String::from("a [ label = \"An A\"; ]\n")
        );
    }
}
