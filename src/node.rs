/*!
One-line description.

More detailed description, with

# Example

 */

use crate::{
    style::{NodeAttributes, Styled},
    Edge, Identified, Identifier,
};
use std::fmt::Display;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Node {
    id: Identifier,
    port: Option<Identifier>,
    attributes: Option<NodeAttributes>,
}

#[derive(Clone, Debug)]
pub enum Field {
    Field {
        port: Option<String>,
        text: Option<String>,
    },
    Flip(Vec<Field>),
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

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id_and_port())?;
        if let Some(attributes) = self.attributes() {
            writeln!(f, " {}", attributes)
        } else {
            writeln!(f)
        }
    }
}

impl From<Identifier> for Node {
    fn from(id: Identifier) -> Self {
        Self::new(id)
    }
}

impl Identified for Node {
    fn id(&self) -> &Identifier {
        &self.id
    }

    fn set_id(self, id: Identifier) -> Self
    where
        Self: Sized,
    {
        let mut self_mut = self;
        self_mut.id = id;
        self_mut
    }
}

impl Styled<NodeAttributes> for Node {
    fn attributes(&self) -> Option<&NodeAttributes> {
        self.attributes.as_ref()
    }

    fn set_attributes(self, attributes: NodeAttributes) -> Self
    where
        Self: Sized,
    {
        let mut self_mut = self;
        self_mut.attributes = Some(attributes);
        self_mut
    }
}

impl Node {
    pub fn new(id: Identifier) -> Self {
        Self {
            id,
            port: Default::default(),
            attributes: Default::default(),
        }
    }

    pub fn port(&self) -> Option<&Identifier> {
        self.port.as_ref()
    }

    pub fn set_port(self, port: Identifier) -> Self {
        let mut self_mut = self;
        self_mut.port = Some(port);
        self_mut
    }

    pub fn unset_port(self) -> Self {
        let mut self_mut = self;
        self_mut.port = None;
        self_mut
    }

    pub fn edge_to(&self, other: &Self) -> Edge {
        Edge::new_from(self, other)
    }

    pub fn edge_from(&self, other: &Self) -> Edge {
        Edge::new_from(other, self)
    }

    pub fn id_and_port(&self) -> String {
        if let Some(port) = self.port() {
            format!("{}:{}", self.id(), port)
        } else {
            self.id().to_string()
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Field { port, text } => format!(
                    "{}{}{}",
                    port.as_ref()
                        .map(|v| format!("<{}>", v))
                        .unwrap_or_default(),
                    if port.is_some() && text.is_some() {
                        " "
                    } else {
                        ""
                    },
                    text.as_ref()
                        .map(|v| if v.chars().any(|c| c.is_whitespace()) {
                            format!("{:?}", v)
                        } else {
                            format!("{}", v)
                        })
                        .unwrap_or_default()
                ),
                Self::Flip(fields) => format!("{{ {} }}", Self::vec_to_string(fields)),
            }
        )
    }
}

impl Field {
    pub fn empty() -> Self {
        Field::Field {
            port: None,
            text: None,
        }
    }

    pub fn port(port: &str) -> Self {
        Field::Field {
            port: Some(port.to_string()),
            text: None,
        }
    }

    pub fn with_text(text: &str) -> Self {
        Field::Field {
            port: None,
            text: Some(text.to_string()),
        }
    }

    pub fn port_with_text(port: &str, text: &str) -> Self {
        Field::Field {
            port: Some(port.to_string()),
            text: Some(text.to_string()),
        }
    }

    fn vec_to_string(fields: &[Field]) -> String {
        let str = fields
            .iter()
            .map(Field::to_string)
            .collect::<Vec<String>>()
            .join(" | ");
        if str.contains('"') {
            format!("\"{}\"", str)
        } else {
            str
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
    use crate::style::{LabelString, NodeAttributes, Styled};
    use crate::{node::Field, Identifier, Node};
    use std::str::FromStr;

    #[test]
    fn test_simple_nodes() {
        assert_eq!(
            Node::new(Identifier::new_unchecked("a")).to_string(),
            String::from("a\n")
        );
        assert_eq!(
            Node::new(Identifier::new_unchecked("a"),)
                .set_attributes(
                    NodeAttributes::default().label(LabelString::from_str("An A").unwrap())
                )
                .to_string(),
            String::from("a [ label = \"An A\" ]\n")
        );
    }

    #[test]
    fn test_field_empty() {
        assert_eq!(Field::empty().to_string(), String::from(""));
    }

    #[test]
    fn test_field_with_port() {
        assert_eq!(Field::port("id").to_string(), String::from("<id>"));
    }

    #[test]
    fn test_field_with_text() {
        assert_eq!(Field::with_text("hello").to_string(), String::from("hello"));
    }

    #[test]
    fn test_field_with_both() {
        assert_eq!(
            Field::port_with_text("id", "hello").to_string(),
            String::from("<id> hello")
        );
    }

    #[test]
    fn test_doc_example() {
        assert_eq!(
            Field::vec_to_string(&vec![
                Field::with_text("hello&#92;nworld"),
                Field::Flip(vec![
                    Field::with_text("b"),
                    Field::Flip(vec![
                        Field::with_text("c"),
                        Field::port_with_text("here", "d"),
                        Field::with_text("e"),
                    ]),
                    Field::with_text("f"),
                ]),
                Field::with_text("g"),
                Field::with_text("h"),
            ]),
            String::from("hello&#92;nworld | { b | { c | <here> d | e } | f } | g | h")
        );
    }
}
