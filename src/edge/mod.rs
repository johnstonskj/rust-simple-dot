/*!
One-line description.

More detailed description, with

# Example

 */

use crate::{
    attributes::{EdgeAttributes, Styled},
    Identified, Identifier, Node,
};
use std::fmt::Display;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Edge {
    head: Identifier,
    tail: Identifier,
    directed: bool,
    attributes: Option<EdgeAttributes>,
}

// #[derive(Clone, Debug)]
// pub struct End {
//     id: Identifier,
//     attributes: Option<EndAttributes>,
// }

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_no_attributes())?;
        if let Some(attributes) = self.attributes() {
            writeln!(f, " {}", attributes)
        } else {
            writeln!(f)
        }
    }
}

impl From<(Identifier, Identifier)> for Edge {
    fn from(pair: (Identifier, Identifier)) -> Self {
        Self::new(pair.0, pair.1)
    }
}

impl Styled<EdgeAttributes> for Edge {
    fn attributes(&self) -> Option<&EdgeAttributes> {
        self.attributes.as_ref()
    }

    fn set_attributes(self, attributes: EdgeAttributes) -> Self
    where
        Self: Sized,
    {
        let mut self_mut = self;
        self_mut.attributes = Some(attributes);
        self_mut
    }
}

impl Edge {
    pub fn new(head: Identifier, tail: Identifier) -> Self {
        Self {
            head,
            tail,
            attributes: Default::default(),
            directed: Default::default(),
        }
    }

    pub fn new_from(head: &Node, tail: &Node) -> Self {
        Self::new(head.id().clone(), tail.id().clone())
    }

    pub fn chain(nodes: &[Node]) -> Vec<Edge> {
        if nodes.len() < 2 {
            Vec::new()
        } else {
            nodes
                .windows(2)
                .map(|pair| Edge::new_from(&pair[0], &pair[1]))
                .collect()
        }
    }

    pub fn circular_chain(nodes: &[Node]) -> Vec<Edge> {
        if nodes.is_empty() {
            Vec::new()
        } else {
            let mut edges: Vec<Edge> = nodes
                .windows(2)
                .map(|pair| Edge::new_from(&pair[0], &pair[1]))
                .collect();
            edges.push(Edge::new_from(
                nodes.first().unwrap(),
                nodes.last().unwrap(),
            ));
            edges
        }
    }

    pub fn head(&self) -> &Identifier {
        &self.head
    }

    pub fn tail(&self) -> &Identifier {
        &self.tail
    }

    pub fn is_directed(&self) -> bool {
        self.directed
    }

    pub(crate) fn set_directed(self, directed: bool) -> Self {
        let mut self_mut = self;
        self_mut.directed = directed;
        self_mut
    }

    pub fn to_string_no_attributes(&self) -> String {
        format!(
            "{} {} {}",
            self.head(),
            if self.directed { "->" } else { "--" },
            self.tail()
        )
    }
}

// ------------------------------------------------------------------------------------------------

// impl From<Identifier> for End {
//     fn from(id: Identifier) -> Self {
//         Self::new(id)
//     }
// }

// impl Identified for End {
//     fn id(&self) -> &Identifier {
//         &self.id
//     }

//     fn set_id(self, id: Identifier) -> Self
//     where
//         Self: Sized,
//     {
//         let mut self_mut = self;
//         self_mut.id = id;
//         self_mut
//     }
// }

// impl Styled<EndAttributes> for End {
//     fn attributes(&self) -> Option<&EndAttributes> {
//         self.attributes.as_ref()
//     }

//     fn set_attributes(self, attributes: EndAttributes) -> Self
//     where
//         Self: Sized,
//     {
//         let mut self_mut = self;
//         self_mut.attributes = Some(attributes);
//         self_mut
//     }
// }

// impl End {
//     pub fn new(id: Identifier) -> Self {
//         Self {
//             id,
//             attributes: Default::default(),
//         }
//     }
// }

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::{Edge, Identifier};

    #[test]
    fn test_undirected_edge_to_string() {
        let a = Identifier::new_unchecked("a");
        let b = Identifier::new_unchecked("b");

        assert_eq!(
            Edge::new(a.clone(), b.clone()).to_string(),
            String::from("a -- b\n")
        );
    }

    #[test]
    fn test_directed_edge_to_string() {
        let a = Identifier::new_unchecked("a");
        let b = Identifier::new_unchecked("b");

        assert_eq!(
            Edge::new(a.clone(), b.clone())
                .set_directed(true)
                .to_string(),
            String::from("a -> b\n")
        );
    }
}
