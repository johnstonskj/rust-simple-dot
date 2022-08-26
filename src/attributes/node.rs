/*!
One-line description.

More detailed description, with

# Example

 */

use crate::attributes::{
    Attribute, Attributes, Color, ColorOrList, Degrees, Double, EscapedString, FontName,
    ImagePosition, Int, LabelLocation, LabelString, LayerRange, NodeStyles, Ordering, Point,
    PointOrBoth, Position, Positive, PositiveNonZero, Rectangle, Shape, Style, Unsigned,
};
use std::fmt::Display;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct NodeAttributes(Vec<Attribute>);

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

styled_common_impl!(NodeAttributes);

impl NodeAttributes {
    attribute_setter!(area, Area, PositiveNonZero);
    attribute_setter!(class, Class, Vec<String>);
    attribute_setter!(color, Color, ColorOrList);
    attribute_setter!(color_scheme, ColorScheme, String);
    attribute_setter!(comment, Comment, String);
    attribute_setter!(distortion, Distortion, Double);
    attribute_setter!(fill_color, FillColor, ColorOrList);
    boolean_attribute_setter!(has_fixed_size, FixedSize);
    attribute_setter!(font_color, FontColor, Color);
    attribute_setter!(font_name, FontName);
    attribute_setter!(font_size, FontSize, Double);
    attribute_setter!(gradient_angle, GradientAngle, Int);
    attribute_setter!(group, Group, String);
    attribute_setter!(height, Height, Positive);
    attribute_setter!(id, Id, EscapedString);
    attribute_setter!(image, Image, String);
    attribute_setter!(image_position, ImagePosition);
    boolean_attribute_setter!(image_scale_to_node, ImageScale);
    attribute_setter!(label, Label, LabelString);
    attribute_setter!(label_location, LabelLocation);
    attribute_setter!(layer, Layer, LayerRange);
    attribute_setter!(margin, Margin, PointOrBoth);
    boolean_attribute_setter!(no_justification, NoJustification);
    attribute_setter!(ordering, Ordering);
    attribute_setter!(orientation, Orientation, Degrees);
    attribute_setter!(pen_width, PenWidth, Positive);
    attribute_setter!(peripheries, Peripheries, Unsigned);
    boolean_attribute_setter!(pin_position, Pin);
    attribute_setter!(position, Position);
    attribute_setter!(record_rectangles, RecordRectangles, Rectangle);
    boolean_attribute_setter!(force_regular, Regular);
    boolean_attribute_setter!(is_root_node, Root);
    attribute_setter!(sample_points, SamplePoints, Unsigned);
    attribute_setter!(shape, Shape);
    attribute_setter!(show_boxes, ShowBoxes, Unsigned);
    attribute_setter!(sides, Sides, Unsigned);
    attribute_setter!(skew, Skew, Double);
    attribute_setter!(sort_value, SortValue, Unsigned);
    style_attribute_setter!(Node, NodeStyles);
    attribute_setter!(target, Target, EscapedString);
    attribute_setter!(tooltip, Tooltip, EscapedString);
    attribute_setter!(url, Url, EscapedString);
    attribute_setter!(vertices, Vertices, Vec<Point>);
    attribute_setter!(width, Width, Positive);
    attribute_setter!(external_label, ExternalLabel, LabelString);
    attribute_setter!(external_label_position, ExternalLabelPosition, Point);
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
