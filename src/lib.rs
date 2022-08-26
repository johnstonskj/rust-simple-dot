/*!
One-line description.

More detailed description, with

# Example

# Features

 */

#![warn(
    unknown_lints,
    // ---------- Stylistic
    absolute_paths_not_starting_with_crate,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    macro_use_extern_crate,
    nonstandard_style, /* group */
    noop_method_call,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    // ---------- Future
    future_incompatible, /* group */
    rust_2021_compatibility, /* group */
    // ---------- Public
    missing_debug_implementations,
    // missing_docs,
    unreachable_pub,
    // ---------- Unsafe
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    // ---------- Unused
    unused, /* group */
)]
#![deny(
    // ---------- Public
    exported_private_dependencies,
    private_in_public,
    // ---------- Deprecated
    anonymous_parameters,
    bare_trait_objects,
    ellipsis_inclusive_range_patterns,
    // ---------- Unsafe
    deref_nullptr,
    drop_bounds,
    dyn_drop,
)]

use unique_id::sequence::SequenceGenerator;
use unique_id::Generator;

#[macro_use]
mod macros;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait Identified {
    fn id(&self) -> &Identifier;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Identifier(String);

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

string_newtype!(Identifier, "Identifier", is_valid_id_string);

impl From<i64> for Identifier {
    fn from(v: i64) -> Self {
        Self::new_unchecked(&v.to_string())
    }
}

impl From<usize> for Identifier {
    fn from(v: usize) -> Self {
        Self::new_unchecked(&v.to_string())
    }
}

impl From<f64> for Identifier {
    fn from(v: f64) -> Self {
        Self::new_unchecked(&v.to_string())
    }
}

impl Identifier {
    pub fn new_unchecked(s: &str) -> Self {
        Self(s.to_string())
    }

    pub fn new_node() -> Self {
        Self(format!("n{}", SequenceGenerator::default().next_id()))
    }

    pub fn new_edge() -> Self {
        Self(format!("e{}", SequenceGenerator::default().next_id()))
    }

    pub fn new_graph() -> Self {
        Self(format!("G{}", SequenceGenerator::default().next_id()))
    }

    pub fn new_cluster_graph() -> Self {
        Self(format!("C{}", SequenceGenerator::default().next_id()))
    }

    pub fn prefix(self, prefix: Identifier) -> Self {
        Self(format!("{}{}", prefix, self))
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[inline]
fn is_valid_id_string(s: &str) -> bool {
    let mut chars = s.chars();
    if let Some(first) = chars.next() {
        if first.is_ascii_alphabetic() || first == '_' {
            chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
        } else if first.is_ascii_digit() || first == '-' || first == '.' {
            chars.all(|c| c.is_ascii_digit() || c == '.')
        } else if first == '"' && s.ends_with('"') {
            true
        } else if first == '<' && s.ends_with('>') {
            true
        } else {
            false
        }
    } else {
        false
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod error;

pub mod graph;
pub use graph::RootGraph;

pub mod node;
pub use node::Node;

pub mod edge;
pub use edge::Edge;

pub mod attributes;

pub mod visitor;

pub mod writer;
