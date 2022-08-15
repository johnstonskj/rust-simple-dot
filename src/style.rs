/*!
One-line description.

More detailed description, with

# Example

 */

use paste::paste;
use std::fmt::Display;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct GraphOptions {}

#[derive(Clone, Debug)]
pub struct GraphStyle {}

#[derive(Clone, Debug)]
pub struct NodeStyle {}

#[derive(Clone, Debug)]
pub struct EdgeStyle {
    arrow_head: Option<ArrowType>,
    arrow_size: Option<f32>,
    arrow_tail: Option<ArrowType>,
}

// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct ArrowType {
    clip_side: ClipSide,
    open: bool,
    shape: ArrowShape,
    next: Option<Box<ArrowType>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArrowShape {
    Box,
    Crow,
    Curve,
    Diamond,
    Dot,
    InverseCurve,
    Inverse,
    None,
    Normal,
    Tee,
    Vee,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClipSide {
    None,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirType {
    None,
    Forward,
    Back,
    Both,
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

impl Default for GraphOptions {
    fn default() -> Self {
        Self {}
    }
}

display_to_inner!(GraphOptions);

impl GraphOptions {
    pub(crate) fn inner_fmt(
        &self,
        _f: &mut std::fmt::Formatter<'_>,
        _indent_level: u32,
        _in_block: bool,
    ) -> std::fmt::Result {
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for GraphStyle {
    fn default() -> Self {
        Self {}
    }
}

display_to_inner!(GraphStyle);

impl GraphStyle {
    pub(crate) fn inner_fmt(
        &self,
        _f: &mut std::fmt::Formatter<'_>,
        _indent_level: u32,
        _in_block: bool,
    ) -> std::fmt::Result {
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for NodeStyle {
    fn default() -> Self {
        Self {}
    }
}

display_to_inner!(NodeStyle);

impl NodeStyle {
    pub(crate) fn inner_fmt(
        &self,
        _f: &mut std::fmt::Formatter<'_>,
        _indent_level: u32,
        _in_block: bool,
    ) -> std::fmt::Result {
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for EdgeStyle {
    fn default() -> Self {
        Self {
            arrow_head: Default::default(),
            arrow_size: Default::default(),
            arrow_tail: Default::default(),
        }
    }
}

display_to_inner!(EdgeStyle);

impl EdgeStyle {
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        self.arrow_head.is_none() && self.arrow_size.is_none() && self.arrow_tail.is_none()
    }

    optional_builder_property!(arrow_head, ArrowType);
    optional_builder_property!(arrow_size, f32);
    optional_builder_property!(arrow_tail, ArrowType);

    write_optional_property!(arrow_head => "arrowhead");
    write_optional_property!(arrow_size => "arrowsize");
    write_optional_property!(arrow_tail => "arrowtail");

    pub(crate) fn inner_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        indent_level: u32,
        in_block: bool,
    ) -> std::fmt::Result {
        if !self.is_default() {
            self.write_arrow_head(f, indent_level, in_block)?;
            self.write_arrow_size(f, indent_level, in_block)?;
            self.write_arrow_tail(f, indent_level, in_block)?;
        }
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

impl Display for ArrowType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_open() {
            write!(f, "o")?;
        }
        write!(f, "{}{}", self.clipping(), self.shape())?;
        if let Some(next_shape) = self.next_shape() {
            write!(f, "{}", next_shape)?;
        }
        Ok(())
    }
}

impl Default for ArrowType {
    fn default() -> Self {
        Self {
            clip_side: Default::default(),
            open: Default::default(),
            shape: Default::default(),
            next: Default::default(),
        }
    }
}

impl ArrowType {
    pub fn new(clip_side: ClipSide, open: bool, shape: ArrowShape) -> Self {
        Self {
            clip_side,
            open,
            shape,
            next: Default::default(),
        }
    }

    pub fn a_box() -> Self {
        Self {
            shape: ArrowShape::Box,
            ..Default::default()
        }
    }

    pub fn crow() -> Self {
        Self {
            shape: ArrowShape::Crow,
            ..Default::default()
        }
    }

    pub fn curve() -> Self {
        Self {
            shape: ArrowShape::Curve,
            ..Default::default()
        }
    }

    pub fn diamond() -> Self {
        Self {
            shape: ArrowShape::Diamond,
            ..Default::default()
        }
    }

    pub fn dot() -> Self {
        Self {
            shape: ArrowShape::Dot,
            ..Default::default()
        }
    }

    pub fn inverse_curve() -> Self {
        Self {
            shape: ArrowShape::InverseCurve,
            ..Default::default()
        }
    }

    pub fn inverse() -> Self {
        Self {
            shape: ArrowShape::Inverse,
            ..Default::default()
        }
    }

    pub fn none() -> Self {
        Self {
            shape: ArrowShape::None,
            ..Default::default()
        }
    }

    pub fn normal() -> Self {
        Self {
            shape: ArrowShape::Normal,
            ..Default::default()
        }
    }

    pub fn tee() -> Self {
        Self {
            shape: ArrowShape::Tee,
            ..Default::default()
        }
    }

    pub fn vee() -> Self {
        Self {
            shape: ArrowShape::Vee,
            ..Default::default()
        }
    }

    pub fn open(mut self) -> Self {
        self.open = true;
        self
    }

    pub fn filled(mut self) -> Self {
        self.open = false;
        self
    }

    pub fn clip_to_left(mut self) -> Self {
        self.clip_side = ClipSide::Left;
        self
    }

    pub fn clip_to_right(mut self) -> Self {
        self.clip_side = ClipSide::Right;
        self
    }

    pub fn no_clipping(mut self) -> Self {
        self.clip_side = ClipSide::None;
        self
    }

    pub fn set_next_shape(mut self, next_shape: ArrowType) -> Self {
        self.next = Some(Box::new(next_shape));
        self
    }

    pub fn unset_next_shape(mut self) -> Self {
        self.next = None;
        self
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn clipping(&self) -> ClipSide {
        self.clip_side
    }

    pub fn shape(&self) -> ArrowShape {
        self.shape
    }

    pub fn next_shape(&self) -> Option<&ArrowType> {
        self.next.as_ref().map(|b| b.as_ref())
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for ArrowShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Box => "box",
                Self::Crow => "crow",
                Self::Curve => "curve",
                Self::Diamond => "diamond",
                Self::Dot => "dot",
                Self::InverseCurve => "icurve",
                Self::Inverse => "inv",
                Self::None => "none",
                Self::Normal => "normal",
                Self::Tee => "tee",
                Self::Vee => "vee",
            }
        )
    }
}

impl Default for ArrowShape {
    fn default() -> Self {
        Self::Normal
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for ClipSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "",
                Self::Left => "l",
                Self::Right => "r",
            }
        )
    }
}

impl Default for ClipSide {
    fn default() -> Self {
        Self::None
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for DirType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "none",
                Self::Forward => "forward",
                Self::Back => "back",
                Self::Both => "both",
            }
        )
    }
}

impl Default for DirType {
    fn default() -> Self {
        Self::None
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
    fn test_arrow_types() {
        assert_eq!(ArrowType::default().to_string(), String::from("normal"));
        assert_eq!(ArrowType::a_box().to_string(), String::from("box"));
        assert_eq!(ArrowType::a_box().open().to_string(), String::from("obox"));
        assert_eq!(
            ArrowType::diamond().open().clip_to_left().to_string(),
            String::from("oldiamond")
        );
        assert_eq!(
            ArrowType::tee()
                .clip_to_left()
                .set_next_shape(ArrowType::diamond().open().clip_to_left())
                .to_string(),
            String::from("lteeoldiamond")
        );
    }

    #[test]
    fn test_edge_style_default() {
        assert_eq!(EdgeStyle::default().to_string(), String::from(""));
    }

    #[test]
    fn test_edge_style() {
        assert_eq!(
            EdgeStyle::default()
                .set_arrow_head(ArrowType::a_box())
                .set_arrow_tail(ArrowType::a_box())
                .to_string(),
            String::from("arrowhead = box; arrowtail = box; ")
        );
    }
}
