/*!
One-line description.

More detailed description, with

# Example

 */

use crate::attributes::{
    AspectRatio, Attribute, Attributes, Color, ColorOrList, Degrees, DistanceMatrix, Double,
    EscapedString, FontName, FontNameMapping, GraphStyles, Int, LabelJustification, LabelLocation,
    LabelString, LayerList, LayerRange, LayoutOptimization, Ordering, OutputOrder, Overlap,
    PackMode, PageOutputDirection, Point, PointOrBoth, Positive, QuadTreeType, RankDirection,
    RankSeparation, RankType, Rectangle, Smoothing, Splines, Start, Style, Unsigned, ViewPort,
};
use std::fmt::Display;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct GraphAttributes(Vec<Attribute>);

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

styled_common_impl!(GraphAttributes);

impl GraphAttributes {
    attribute_setter!(background, Background, String);
    attribute_setter!(bounding_box, BoundingBox, Rectangle);
    attribute_setter!(background_color, BackgroundColor, ColorOrList);
    boolean_attribute_setter!(is_centered, Center);
    attribute_setter!(charset, Charset, String);
    attribute_setter!(class, Class, Vec<String>);
    attribute_setter!(color_scheme, ColorScheme, String);
    attribute_setter!(comment, Comment, String);
    boolean_attribute_setter!(is_compound, Compound);
    boolean_attribute_setter!(concentrate_edges, Concentrate);
    attribute_setter!(damping, Damping, Positive);
    attribute_setter!(default_distance, DefaultDistance, Double);
    attribute_setter!(layout_dimensions, DimensionsLayout, Int);
    attribute_setter!(render_dimensions, DimensionsRender, Int);
    attribute_setter!(directed_edge_constraints, DirEdgeConstraints, String);
    attribute_setter!(dpi, Dpi, Positive);
    attribute_setter!(epsilon, Epsilon, Double);
    attribute_setter!(font_color, FontColor, Color);
    attribute_setter!(font_name, FontName);
    attribute_setter!(font_names_for_svg, FontNames, FontNameMapping);
    attribute_setter!(font_path, FontPath, String);
    attribute_setter!(font_size, FontSize, Double);
    boolean_attribute_setter!(force_external_labels, ForceLabels);
    attribute_setter!(gradient_angle, GradientAngle, Int);
    attribute_setter!(id, Id, EscapedString);
    attribute_setter!(image_path, ImagePath, String);
    attribute_setter!(input_scale, InputScale, Double);
    attribute_setter!(spring_constant, SpringConstant, Positive);
    attribute_setter!(label, Label, LabelString);
    attribute_setter!(label_height, LabelHeight, Double);
    attribute_setter!(label_justification, LabelJustification);
    attribute_setter!(label_location, LabelLocation);
    attribute_setter!(label_position, LabelPosition, Point);
    attribute_setter!(label_scheme, LabelScheme, Int);
    attribute_setter!(label_width, LabelWidth, Double);
    boolean_attribute_setter!(is_landscape, Landscape);
    attribute_setter!(layers, Layers, LayerList);
    attribute_setter!(layer_select, LayerSelect, LayerRange);
    attribute_setter!(layout_engine, LayoutEngine, String);
    attribute_setter!(levels, Levels, Int);
    attribute_setter!(levels_gap, LevelsGap, Double);
    attribute_setter!(margin, Margin, PointOrBoth);
    attribute_setter!(max_iterations, MaxIterations, Int);
    attribute_setter!(mc_limit, McLimit, Double);
    attribute_setter!(min_distance, MinDistance, Positive);
    attribute_setter!(mode, Mode, LayoutOptimization);
    attribute_setter!(model, Model, DistanceMatrix);
    boolean_attribute_setter!(use_mosek, Mosek);
    boolean_attribute_setter!(use_new_ranking, NewRank);
    attribute_setter!(node_separation, NodeSeparation, Positive);
    boolean_attribute_setter!(no_justification, NoJustification);
    attribute_setter!(normalize, Normalize, Double);
    boolean_attribute_setter!(no_layout_translation, NoTranslate);
    attribute_setter!(ns_limit, NetworkSimplexLimit, Double);
    attribute_setter!(ns_limit_1, NetworkSimplexLimit1, Double);
    attribute_setter!(ordering, Ordering);
    attribute_setter!(orientation, Orientation, Degrees);
    attribute_setter!(output_order, OutputOrder);
    attribute_setter!(overlap, Overlap);
    attribute_setter!(overlap_scaling, OverlapScaling, Double);
    boolean_attribute_setter!(no_overlap_shrink, OverlapShrink);
    boolean_attribute_setter!(pack_graphs, Pack);
    attribute_setter!(pack_mode, PackMode);
    attribute_setter!(pad, Pad, PointOrBoth);
    attribute_setter!(page_size, PageSize, PointOrBoth);
    attribute_setter!(page_output_direction, PageOutputDirection);
    attribute_setter!(quad_tree, QuadTree, QuadTreeType);
    attribute_setter!(quantum, Quantum, Positive);
    attribute_setter!(rank_direction, RankDirection);
    attribute_setter!(rank, Rank, RankType);
    attribute_setter!(rank_separation, RankSeparation);
    attribute_setter!(aspect_ratio, AspectRatio);
    boolean_attribute_setter!(rerun_crossing_minimization, RerunCrossingMinimization);
    attribute_setter!(repulsive_force, RepulsiveForce, Positive);
    attribute_setter!(resolution, Resolution, Positive);
    attribute_setter!(root, RootRef, String);
    attribute_setter!(rotate, Rotate, Int);
    attribute_setter!(rotation, Rotation, Positive);
    attribute_setter!(scale, Scale, PointOrBoth);
    attribute_setter!(search_size, SearchSize, Int);
    attribute_setter!(separation, Separation, PointOrBoth);
    attribute_setter!(show_boxes, ShowBoxes, Unsigned);
    attribute_setter!(size, Size, PointOrBoth);
    attribute_setter!(smoothing, Smoothing);
    attribute_setter!(sort_value, SortValue, Unsigned);
    attribute_setter!(splines, Splines);
    attribute_setter!(start, Start);
    attribute_setter!(stylesheet, Stylesheet, String);
    attribute_setter!(target, Target, EscapedString);
    attribute_setter!(tooltip, Tooltip, EscapedString);
    attribute_setter!(true_color, TrueColor, bool);
    attribute_setter!(url, Url, EscapedString);
    attribute_setter!(view_port, ViewPort);
    attribute_setter!(voronoi_margin, VoronoiMargin, Positive);
    attribute_setter!(xdot_version, XDotVersion, String);

    style_attribute_setter!(Graph, GraphStyles);
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
