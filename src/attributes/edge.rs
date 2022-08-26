/*!
One-line description.

More detailed description, with

# Example

 */

use crate::attributes::{
    ArrowType, Attribute, Attributes, Color, ColorOrList, Direction, Double, EdgeStyles,
    EscapedString, FontName, LabelString, LayerRange, Point, PortPosition, Position, Positive,
    Style, Unsigned,
};
use std::fmt::Display;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct EdgeAttributes(Vec<Attribute>);

// #[derive(Clone, Debug)]
// pub struct EndAttributes(Vec<Attribute>);

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

styled_common_impl!(EdgeAttributes);

impl EdgeAttributes {
    attribute_setter!(arrow_head, ArrowHead, ArrowType);
    attribute_setter!(arrow_size, ArrowSize, Positive);
    attribute_setter!(arrow_tail, ArrowTail, ArrowType);
    attribute_setter!(class, Class, Vec<String>);
    attribute_setter!(color, Color, ColorOrList);
    attribute_setter!(color_scheme, ColorScheme, String);
    attribute_setter!(comment, Comment, String);
    boolean_attribute_setter!(no_layout_constraint, Constraint);
    boolean_attribute_setter!(decorate_label, Decorate);
    attribute_setter!(direction, Direction, Direction);
    attribute_setter!(edge_target, EdgeTarget, EscapedString);
    attribute_setter!(edge_tooltip, EdgeTooltip, EscapedString);
    attribute_setter!(edge_url, EdgeUrl, EscapedString);
    attribute_setter!(fill_color, FillColor, ColorOrList);
    attribute_setter!(font_color, FontColor, Color);
    attribute_setter!(font_name, FontName, FontName);
    attribute_setter!(font_size, FontSize, Double);
    attribute_setter!(head_label_position, HeadLabelPosition, Point);
    boolean_attribute_setter!(dont_clip_head, HeadClip);
    attribute_setter!(head_label, HeadLabel, LabelString);
    attribute_setter!(head_port, HeadPort, PortPosition);
    attribute_setter!(head_target, HeadTarget, EscapedString);
    attribute_setter!(head_tooltip, HeadTooltip, EscapedString);
    attribute_setter!(head_url, HeadUrl, EscapedString);
    attribute_setter!(id, Id, EscapedString);
    attribute_setter!(label, Label, LabelString);
    attribute_setter!(label_angle, LabelAngle, Double);
    attribute_setter!(label_distance, LabelDistance, Positive);
    boolean_attribute_setter!(allow_label_float, LabelFloat);
    attribute_setter!(label_font_color, LabelFontColor, Color);
    attribute_setter!(label_font_name, LabelFontName, FontName);
    attribute_setter!(label_font_size, LabelFontSize, Double);
    attribute_setter!(label_position, LabelPosition, Point);
    attribute_setter!(label_target, LabelTarget, EscapedString);
    attribute_setter!(label_tooltip, LabelTooltip, EscapedString);
    attribute_setter!(label_url, LabelUrl, EscapedString);
    attribute_setter!(layer, Layer, LayerRange);
    attribute_setter!(length, Length, Double);
    attribute_setter!(logical_head, LogicalHead, String);
    attribute_setter!(logical_tail, LogicalTail, String);
    attribute_setter!(minimum_length, MinLength, Unsigned);
    boolean_attribute_setter!(no_justification, NoJustification);
    attribute_setter!(pen_width, PenWidth, Positive);
    attribute_setter!(position, Position);
    attribute_setter!(same_head, SameHead, String);
    attribute_setter!(same_tail, SameTail, String);
    attribute_setter!(show_boxes, ShowBoxes, Unsigned);
    style_attribute_setter!(Edge, EdgeStyles);
    attribute_setter!(tail_label_position, TailLabelPosition, Point);
    boolean_attribute_setter!(dont_clip_tail, TailClip);
    attribute_setter!(tail_label, TailLabel, LabelString);
    attribute_setter!(tail_port, TailPort, PortPosition);
    attribute_setter!(tail_target, TailTarget, EscapedString);
    attribute_setter!(tail_tooltip, TailTooltip, EscapedString);
    attribute_setter!(tail_url, TailUrl, EscapedString);
    attribute_setter!(target, Target, EscapedString);
    attribute_setter!(tooltip, Tooltip, EscapedString);
    attribute_setter!(url, Url, EscapedString);
    attribute_setter!(weight, Weight, Unsigned);
    attribute_setter!(external_label, ExternalLabel, LabelString);
    attribute_setter!(external_label_position, ExternalLabelPosition, Point);
}

// ------------------------------------------------------------------------------------------------

// styled_common_impl!(EndAttributes);
// impl EndAttributes {}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
