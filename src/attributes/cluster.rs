/*!
One-line description.

More detailed description, with

# Example

 */

use crate::attributes::{
    Attribute, Attributes, ClusterStyles, Color, ColorOrList, Double, EscapedString, FontName, Int,
    LabelJustification, LabelLocation, LabelString, LayerRange, Point, PointOrBoth, Positive,
    PositiveNonZero, RankType, Style, Unsigned,
};
use std::fmt::Display;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct ClusterAttributes(Vec<Attribute>);

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

styled_common_impl!(ClusterAttributes);

impl ClusterAttributes {
    attribute_setter!(area, Area, PositiveNonZero);
    attribute_setter!(background_color, BackgroundColor, ColorOrList);
    attribute_setter!(class, Class, Vec<String>);
    attribute_setter!(color, Color, ColorOrList);
    attribute_setter!(color_scheme, ColorScheme, String);
    attribute_setter!(fill_color, FillColor, ColorOrList);
    attribute_setter!(font_color, FontColor, Color);
    attribute_setter!(font_name, FontName, FontName);
    attribute_setter!(font_size, FontSize, Double);
    attribute_setter!(gradient_angle, GradientAngle, Int);
    attribute_setter!(id, Id, EscapedString);
    attribute_setter!(spring_constant, SpringConstant, Positive);
    attribute_setter!(label, Label, LabelString);
    attribute_setter!(label_height, LabelHeight, Double);
    attribute_setter!(label_justification, LabelJustification);
    attribute_setter!(label_location, LabelLocation);
    attribute_setter!(label_position, LabelPosition, Point);
    attribute_setter!(label_width, LabelWidth, Double);
    attribute_setter!(layer, Layer, LayerRange);
    attribute_setter!(margin, Margin, PointOrBoth);
    boolean_attribute_setter!(no_justification, NoJustification);
    attribute_setter!(pen_width, PenWidth, Positive);
    attribute_setter!(peripheries, Peripheries, Unsigned);
    attribute_setter!(rank, Rank, RankType);
    attribute_setter!(sort_value, SortValue, Unsigned);
    style_attribute_setter!(Cluster, ClusterStyles);
    attribute_setter!(target, Target, EscapedString);
    attribute_setter!(tooltip, Tooltip, EscapedString);
    attribute_setter!(url, Url, EscapedString);
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
