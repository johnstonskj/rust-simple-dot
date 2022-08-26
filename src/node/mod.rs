/*!
One-line description.

More detailed description, with

# Example

 */

use crate::{
    attributes::{NodeAttributes, Styled},
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

pub fn field_vec_to_string(fields: &[Field]) -> String {
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
                Self::Flip(fields) => format!("{{ {} }}", field_vec_to_string(fields)),
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
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
