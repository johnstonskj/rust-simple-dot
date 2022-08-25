/*!
One-line description.

More detailed description, with

# Example

 */

use crate::error::invalid_value;
use std::{fmt::Display, str::FromStr};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait Styled<A>
where
    A: Attributes,
{
    fn attributes(&self) -> Option<&A>;

    fn has_attributes(&self) -> bool {
        self.attributes()
            .map(|v| !v.as_ref().is_empty())
            .unwrap_or(false)
    }

    fn set_attributes(self, attributes: A) -> Self
    where
        Self: Sized;
}

pub trait Attributes: Default + Display + AsRef<Vec<Attribute>> {}

#[derive(Clone, Debug)]
pub struct ClusterAttributes(Vec<Attribute>);

#[derive(Clone, Debug)]
pub struct GraphAttributes(Vec<Attribute>);

#[derive(Clone, Debug)]
pub struct NodeAttributes(Vec<Attribute>);

#[derive(Clone, Debug)]
pub struct EdgeAttributes(Vec<Attribute>);

// #[derive(Clone, Debug)]
// pub struct EndAttributes(Vec<Attribute>);

#[derive(Clone, Debug, PartialEq)]
pub enum Attribute {
    ///
    /// Indicates the preferred area for a node or empty cluster when laid out
    /// by patchwork. patchwork only.
    ///
    /// Default: 1.0
    ///
    /// Used By: NC
    ///
    Area(PositiveNonZero),
    ArrowHead(ArrowType),
    ArrowSize(Positive),
    ArrowTail(ArrowType),
    Background(String),
    BoundingBox(Rectangle),
    BackgroundColor(ColorOrList),
    Center(bool),
    Charset(String),
    Class(Vec<String>),
    ClusterRank(ClusterRank),
    Color(ColorOrList),
    ColorScheme(String),
    Comment(String),
    Compound(bool),
    Concentrate(bool),
    Constraint(bool),
    Damping(Positive),
    Decorate(bool),
    DefaultDistance(Double),
    DimensionsLayout(Int), // `2..=10`
    DimensionsRender(Int), // `2..=10`
    Direction(Direction),
    DirEdgeConstraints(String), // or bool
    Distortion(Double),
    Dpi(Positive),
    EdgeUrl(EscapedString),
    EdgeTarget(EscapedString),
    EdgeTooltip(EscapedString),
    Epsilon(Double),
    EdgeSeparation(Double), // addDouble or addPoint
    FillColor(ColorOrList),
    FixedSize(bool), // or String
    FontColor(Color),
    FontName(FontName),
    FontNames(FontNameMapping),
    FontPath(String),
    FontSize(Double),
    ForceLabels(bool),
    GradientAngle(Int),
    Group(String),
    HeadLabelPosition(Point),
    HeadClip(bool),
    HeadUrl(EscapedString),
    HeadLabel(LabelString),
    HeadPort(PortPosition),
    HeadTarget(EscapedString),
    HeadTooltip(EscapedString),
    Height(Positive), // `0.02..=+Inf.0`
    Id(EscapedString),
    Image(String),
    ImagePath(String),
    ImagePosition(ImagePosition),
    ImageScale(bool), // or String
    InputScale(Double),
    SpringConstant(Positive), // K
    Label(LabelString),
    LabelScheme(Int),   // `00..=MAX`
    LabelAngle(Double), // `-180.0..=-25.0`
    LabelDistance(Positive),
    LabelFloat(bool),
    LabelFontColor(Color),
    LabelFontName(FontName),
    LabelFontSize(Double),
    LabelHeight(Double),
    LabelJustification(LabelJustification),
    LabelLocation(LabelLocation),
    LabelPosition(Point),
    LabelTarget(EscapedString),
    LabelTooltip(EscapedString),
    LabelUrl(EscapedString),
    LabelWidth(Double),
    Landscape(bool),
    Layer(LayerRange),
    Layers(LayerList),
    LayerSelect(LayerRange),
    LayoutEngine(String),
    Length(Double),
    Levels(Int),
    LevelsGap(Double),
    LogicalHead(String),
    LogicalTail(String),
    Margin(PointOrBoth),
    MaxIterations(Int),
    McLimit(Double),
    MinDistance(Positive),
    MinLength(Unsigned),
    Mode(LayoutOptimization),
    Model(DistanceMatrix),
    Mosek(bool),
    NewRank(bool),
    NodeSeparation(Positive),
    NoJustification(bool),
    Normalize(Double),
    NoTranslate(bool),
    NetworkSimplexLimit(Double),
    NetworkSimplexLimit1(Double),
    Ordering(Ordering),
    Orientation(Degrees), // or string "[lL]*"
    OutputOrder(OutputOrder),
    Overlap(Overlap),
    OverlapScaling(Double),
    OverlapShrink(bool),
    Pack(bool), // or i16
    PackMode(PackMode),
    Pad(PointOrBoth),
    PageSize(PointOrBoth),
    PageOutputDirection(PageOutputDirection),
    PenColor(Color),
    PenWidth(Positive),
    Peripheries(Unsigned),
    Pin(bool),
    Position(Position),
    QuadTree(QuadTreeType),
    Quantum(Positive),
    Rank(RankType),
    RankDirection(RankDirection),
    RankSeparation(RankSeparation),
    AspectRatio(AspectRatio),
    RecordRectangles(Rectangle),
    Regular(bool),
    RerunCrossingMinimization(bool),
    RepulsiveForce(Positive),
    Resolution(Positive),
    Root(bool),
    RootRef(String), // also outputs to "root"
    Rotate(Int),
    Rotation(Positive),
    SameHead(String),
    SameTail(String),
    SamplePoints(Unsigned),
    Scale(PointOrBoth),
    SearchSize(Int),
    Separation(PointOrBoth),
    Shape(Shape),
    ShowBoxes(Unsigned),
    Sides(Unsigned),
    Size(PointOrBoth),
    Skew(Double),
    Smoothing(Smoothing),
    SortValue(Unsigned),
    Splines(Splines),
    Start(Start),
    Style(Vec<Style>),
    Stylesheet(String),
    TailLabelPosition(Point),
    TailClip(bool),
    TailLabel(LabelString),
    TailPort(PortPosition),
    TailTarget(EscapedString),
    TailTooltip(EscapedString),
    TailUrl(EscapedString),
    Target(EscapedString),
    Tooltip(EscapedString),
    TrueColor(bool),
    Url(EscapedString),
    Vertices(Vec<Point>),
    ViewPort(ViewPort),
    VoronoiMargin(Positive),
    Weight(Unsigned),
    Width(Positive),
    XDotVersion(String),
    ExternalLabel(LabelString),
    ExternalLabelPosition(Point),
}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

pub type Int = i32;

pub type Unsigned = u32;

/// `-Inf.0..=+Inf.0`
pub type Double = f64;

/// `0.0..=1.0`
#[derive(Clone, Debug, PartialEq)]
pub struct Scale(Double);

/// `0.0..=+Inf.0`
#[derive(Clone, Debug, PartialEq)]
pub struct Positive(Double);

/// `(>0.0)..=+Inf.0`
#[derive(Clone, Debug, PartialEq)]
pub struct PositiveNonZero(Double);

/// `0.0..=360.0`
#[derive(Clone, Debug, PartialEq)]
pub struct Degrees(Double);

#[derive(Clone, Debug, PartialEq)]
pub struct EscapedString(String);

#[derive(Clone, Debug, PartialEq)]
pub struct LabelString(String);

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

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

#[derive(Clone, Debug, PartialEq)]
pub struct ArrowType {
    clip_side: ClipSide,
    open: bool,
    shape: ArrowShape,
    next: Option<Box<ArrowType>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AspectRatio {
    Fill,
    Compress,
    Expand,
    Auto,
    Exactly(Double),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClipSide {
    None,
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ClusterRank {
    Local,
    Global,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ClusterStyles {
    Filled,
    Striped,
    Rounded,
    Radial,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Color {
    Rgb { r: u8, g: u8, b: u8 },
    Rgba { r: u8, g: u8, b: u8, a: u8 },
    Hsv { h: Scale, s: Scale, v: Scale },
    Name(String),
}

pub type ColorList = Vec<WeightedColor>;

#[derive(Clone, Debug, PartialEq)]
pub enum ColorOrList {
    Color(Color),
    List(ColorList),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CompassPoint {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
    Center,
    Appropriate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    None,
    Forward,
    Back,
    Both,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DistanceMatrix {
    Circuit,
    Shortpath,
    Subset,
    Mds,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EdgeStyles {
    Dashed,
    Dotted,
    Solid,
    Invisible,
    Bold,
    Tapered,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FontName {
    family: String,
    slant: Option<FontSlant>,
    weight: Option<FontWeight>,
    separator: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FontNameMapping {
    Svg,
    Postscript,
    FontConfig,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FontSlant {
    Italic,
    Oblique,
    Roman,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FontWeight {
    Light,
    Medium,
    DemiBold,
    Bold,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GraphStyles {
    Filled,
    Radial,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImagePosition {
    TopLeft,
    TopCentered,
    TopRight,
    MiddleLeft,
    MiddleCentered,
    MiddleRight,
    BottomLeft,
    BottomCentered,
    BottomRight,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LabelJustification {
    Left,
    Centered,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LabelLocation {
    Top,
    Centered,
    Bottom,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LayerList {
    layers: Vec<String>,
    separator: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LayerRange {
    layers: Vec<LayerRangeValue>,
    separator: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LayerRangeValue {
    Range(LayerRange),
    Index(Unsigned),
    All,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LayoutOptimization {
    GradientDescent,
    StressMajorization,
    StochasticGradientDescent,
    Hierarchical,
    IpSeparation,
    SpringElectrical,
    MaxEnt,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NodeStyles {
    Dashed,
    Dotted,
    Solid,
    Invisible,
    Bold,
    Filled,
    Striped,
    Wedged,
    Diagonals,
    Rounded,
    Radial,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Ordering {
    In,
    Out,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OutputOrder {
    BreadthFirst,
    NodesFirst,
    EdgesFirst,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Overlap {
    True,
    False,
    Scale,
    Prism(u16),
    Voronoi,
    ScaleXy,
    Compress,
    Quadratic,
    Ipsep,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PackMode {
    Node,
    Graph,
    Cluster,
    Array {
        order: Option<PackModeOrder>,
        align: Option<PackModeAlign>,
        count: Option<Unsigned>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PackModeAlign {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PackModeOrder {
    Row,
    Column,
    User,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PageOutputDirection {
    major: RankDirection,
    minor: RankDirection,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    x: Double,
    y: Double,
    fixed: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PointOrBoth {
    Both(Double),
    Point(Point),
}

#[derive(Clone, Debug, PartialEq)]
pub struct PortPosition {
    port_name: String,
    compass_point: Option<CompassPoint>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Position {
    Point(Point),
    SplineType(SplineType),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QuadTreeType {
    Normal,
    Fast,
    None,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RankDirection {
    BottomToTop,
    TopToBottom,
    LeftToRight,
    RightToLeft,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RankSeparation {
    min_vertical_distance: Vec<Double>,
    equally: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RankType {
    Max,
    Min,
    Same,
    Sink,
    Source,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Rectangle {
    lower_left_x: Double,
    lower_left_y: Double,
    upper_right_x: Double,
    upper_right_y: Double,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Shape(String);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Smoothing {
    None,
    AverageDistance,
    GraphDistance,
    PowerDistance,
    Rng,
    Spring,
    Triangle,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Splines {
    None,
    Line,
    Spline,
    Polyline,
    Orthogonal,
    Curved,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SplineType {
    start: Option<Point>,
    end: Option<Point>,
    point: Point,
    triples: Vec<Triple>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Start {
    style: Option<StartStyle>,
    seed: Option<Unsigned>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StartStyle {
    Regular,
    AtSelf,
    Random,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Style {
    Node(NodeStyles),
    Edge(EdgeStyles),
    Cluster(ClusterStyles),
    Graph(GraphStyles),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Triple {
    p_1: Double,
    p_2: Double,
    p_3: Double,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ViewPort {
    width: Positive,
    height: Positive,
    zoom: Option<Positive>,
    center: Option<ViewPortCenter>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ViewPortCenter {
    Point(Point),
    Node(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct WeightedColor {
    color: Color,
    weight: Option<Scale>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! styled_common_impl {
    ($type:ty) => {
        impl Default for $type {
            fn default() -> Self {
                Self(Default::default())
            }
        }

        impl Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if self.0.is_empty() {
                    write!(f, "[]")
                } else {
                    write!(
                        f,
                        "[ {} ]",
                        self.0
                            .iter()
                            .map(Attribute::to_string)
                            .collect::<Vec<String>>()
                            .join("; ")
                    )
                }
            }
        }

        impl AsRef<Vec<Attribute>> for $type {
            fn as_ref(&self) -> &Vec<Attribute> {
                &self.0
            }
        }

        impl Attributes for $type {}

        impl $type {
            fn push(self, attribute: Attribute) -> Self
            where
                Self: Sized,
            {
                let mut self_mut = self;
                self_mut.0.push(attribute);
                self_mut
            }
        }
    };
}

macro_rules! attribute_setter {
    ($name:ident, $variant_and_type:ident) => {
        pub fn $name(self, $name: $variant_and_type) -> Self {
            self.push(Attribute::$variant_and_type($name))
        }
    };
    ($name:ident, $variant:ident, $type:ty) => {
        pub fn $name(self, $name: $type) -> Self {
            self.push(Attribute::$variant($name))
        }
    };
}

macro_rules! style_attribute_setter {
    ($style_variant:ident, $style_type:ty) => {
        pub fn style(self, styles: Vec<$style_type>) -> Self {
            self.push(Attribute::Style(
                styles
                    .into_iter()
                    .map(|s| Style::$style_variant(s))
                    .collect(),
            ))
        }
    };
}

macro_rules! boolean_attribute_setter {
    ($name_true:ident, $variant:ident) => {
        pub fn $name_true(self) -> Self {
            self.push(Attribute::$variant(true))
        }
    };
}

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
    style_attribute_setter!(Graph, GraphStyles);
    attribute_setter!(stylesheet, Stylesheet, String);
    attribute_setter!(target, Target, EscapedString);
    attribute_setter!(tooltip, Tooltip, EscapedString);
    attribute_setter!(true_color, TrueColor, bool);
    attribute_setter!(url, Url, EscapedString);
    attribute_setter!(view_port, ViewPort);
    attribute_setter!(voronoi_margin, VoronoiMargin, Positive);
    attribute_setter!(xdot_version, XDotVersion, String);
}

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
// ------------------------------------------------------------------------------------------------

impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} = {}",
            self.name(),
            match self {
                Attribute::Area(v) => v.to_string(),
                Attribute::ArrowHead(v) => v.to_string(),
                Attribute::ArrowSize(v) => v.to_string(),
                Attribute::ArrowTail(v) => v.to_string(),
                Attribute::Background(v) => v.to_string(),
                Attribute::BoundingBox(v) => v.to_string(),
                Attribute::BackgroundColor(v) => v.to_string(),
                Attribute::Center(v) => v.to_string(),
                Attribute::Charset(v) => v.to_string(),
                Attribute::Class(v) => v.join(" "),
                Attribute::ClusterRank(v) => v.to_string(),
                Attribute::Color(v) => v.to_string(),
                Attribute::ColorScheme(v) => v.to_string(),
                Attribute::Comment(v) => v.to_string(),
                Attribute::Compound(v) => v.to_string(),
                Attribute::Concentrate(v) => v.to_string(),
                Attribute::Constraint(v) => v.to_string(),
                Attribute::Damping(v) => v.to_string(),
                Attribute::Decorate(v) => v.to_string(),
                Attribute::DefaultDistance(v) => v.to_string(),
                Attribute::DimensionsLayout(v) => v.to_string(),
                Attribute::DimensionsRender(v) => v.to_string(),
                Attribute::Direction(v) => v.to_string(),
                Attribute::DirEdgeConstraints(v) => v.to_string(),
                Attribute::Distortion(v) => v.to_string(),
                Attribute::Dpi(v) => v.to_string(),
                Attribute::EdgeUrl(v) => v.to_string(),
                Attribute::EdgeTarget(v) => v.to_string(),
                Attribute::EdgeTooltip(v) => v.to_string(),
                Attribute::Epsilon(v) => v.to_string(),
                Attribute::EdgeSeparation(v) => v.to_string(),
                Attribute::FillColor(v) => v.to_string(),
                Attribute::FixedSize(v) => v.to_string(),
                Attribute::FontColor(v) => v.to_string(),
                Attribute::FontName(v) => v.to_string(),
                Attribute::FontNames(v) => v.to_string(),
                Attribute::FontPath(v) => v.to_string(),
                Attribute::FontSize(v) => v.to_string(),
                Attribute::ForceLabels(v) => v.to_string(),
                Attribute::GradientAngle(v) => v.to_string(),
                Attribute::Group(v) => v.to_string(),
                Attribute::HeadLabelPosition(v) => v.to_string(),
                Attribute::HeadClip(v) => v.to_string(),
                Attribute::HeadUrl(v) => v.to_string(),
                Attribute::HeadLabel(v) => v.to_string(),
                Attribute::HeadPort(v) => v.to_string(),
                Attribute::HeadTarget(v) => v.to_string(),
                Attribute::HeadTooltip(v) => v.to_string(),
                Attribute::Height(v) => v.to_string(),
                Attribute::Id(v) => v.to_string(),
                Attribute::Image(v) => v.to_string(),
                Attribute::ImagePath(v) => v.to_string(),
                Attribute::ImagePosition(v) => v.to_string(),
                Attribute::ImageScale(v) => v.to_string(),
                Attribute::InputScale(v) => v.to_string(),
                Attribute::SpringConstant(v) => v.to_string(),
                Attribute::Label(v) => v.to_string(),
                Attribute::LabelScheme(v) => v.to_string(),
                Attribute::LabelAngle(v) => v.to_string(),
                Attribute::LabelDistance(v) => v.to_string(),
                Attribute::LabelFloat(v) => v.to_string(),
                Attribute::LabelFontColor(v) => v.to_string(),
                Attribute::LabelFontName(v) => v.to_string(),
                Attribute::LabelFontSize(v) => v.to_string(),
                Attribute::LabelJustification(v) => v.to_string(),
                Attribute::LabelLocation(v) => v.to_string(),
                Attribute::LabelTarget(v) => v.to_string(),
                Attribute::LabelTooltip(v) => v.to_string(),
                Attribute::LabelUrl(v) => v.to_string(),
                Attribute::Landscape(v) => v.to_string(),
                Attribute::Layer(v) => v.to_string(),
                Attribute::Layers(v) => v.to_string(),
                Attribute::LayerSelect(v) => v.to_string(),
                Attribute::LayoutEngine(v) => v.to_string(),
                Attribute::Length(v) => v.to_string(),
                Attribute::Levels(v) => v.to_string(),
                Attribute::LevelsGap(v) => v.to_string(),
                Attribute::LabelHeight(v) => v.to_string(),
                Attribute::LabelPosition(v) => v.to_string(),
                Attribute::LabelWidth(v) => v.to_string(),
                Attribute::LogicalHead(v) => v.to_string(),
                Attribute::LogicalTail(v) => v.to_string(),
                Attribute::Margin(v) => v.to_string(),
                Attribute::MaxIterations(v) => v.to_string(),
                Attribute::McLimit(v) => v.to_string(),
                Attribute::MinDistance(v) => v.to_string(),
                Attribute::MinLength(v) => v.to_string(),
                Attribute::Mode(v) => v.to_string(),
                Attribute::Model(v) => v.to_string(),
                Attribute::Mosek(v) => v.to_string(),
                Attribute::NewRank(v) => v.to_string(),
                Attribute::NodeSeparation(v) => v.to_string(),
                Attribute::NoJustification(v) => v.to_string(),
                Attribute::Normalize(v) => v.to_string(),
                Attribute::NoTranslate(v) => v.to_string(),
                Attribute::NetworkSimplexLimit(v) => v.to_string(),
                Attribute::NetworkSimplexLimit1(v) => v.to_string(),
                Attribute::Ordering(v) => v.to_string(),
                Attribute::Orientation(v) => v.to_string(),
                Attribute::OutputOrder(v) => v.to_string(),
                Attribute::Overlap(v) => v.to_string(),
                Attribute::OverlapScaling(v) => v.to_string(),
                Attribute::OverlapShrink(v) => v.to_string(),
                Attribute::Pack(v) => v.to_string(),
                Attribute::PackMode(v) => v.to_string(),
                Attribute::Pad(v) => v.to_string(),
                Attribute::PageSize(v) => v.to_string(),
                Attribute::PageOutputDirection(v) => v.to_string(),
                Attribute::PenColor(v) => v.to_string(),
                Attribute::PenWidth(v) => v.to_string(),
                Attribute::Peripheries(v) => v.to_string(),
                Attribute::Pin(v) => v.to_string(),
                Attribute::Position(v) => v.to_string(),
                Attribute::QuadTree(v) => v.to_string(),
                Attribute::Quantum(v) => v.to_string(),
                Attribute::Rank(v) => v.to_string(),
                Attribute::RankDirection(v) => v.to_string(),
                Attribute::RankSeparation(v) => v.to_string(),
                Attribute::AspectRatio(v) => v.to_string(),
                Attribute::RecordRectangles(v) => v.to_string(),
                Attribute::Regular(v) => v.to_string(),
                Attribute::RerunCrossingMinimization(v) => v.to_string(),
                Attribute::RepulsiveForce(v) => v.to_string(),
                Attribute::Resolution(v) => v.to_string(),
                Attribute::Root(v) => v.to_string(),
                Attribute::RootRef(v) => v.to_string(),
                Attribute::Rotate(v) => v.to_string(),
                Attribute::Rotation(v) => v.to_string(),
                Attribute::SameHead(v) => v.to_string(),
                Attribute::SameTail(v) => v.to_string(),
                Attribute::SamplePoints(v) => v.to_string(),
                Attribute::Scale(v) => v.to_string(),
                Attribute::SearchSize(v) => v.to_string(),
                Attribute::Separation(v) => v.to_string(),
                Attribute::Shape(v) => v.to_string(),
                Attribute::ShowBoxes(v) => v.to_string(),
                Attribute::Sides(v) => v.to_string(),
                Attribute::Size(v) => v.to_string(),
                Attribute::Skew(v) => v.to_string(),
                Attribute::Smoothing(v) => v.to_string(),
                Attribute::SortValue(v) => v.to_string(),
                Attribute::Splines(v) => v.to_string(),
                Attribute::Start(v) => v.to_string(),
                Attribute::Style(v) => v
                    .iter()
                    .map(Style::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
                Attribute::Stylesheet(v) => v.to_string(),
                Attribute::TailLabelPosition(v) => v.to_string(),
                Attribute::TailClip(v) => v.to_string(),
                Attribute::TailLabel(v) => v.to_string(),
                Attribute::TailPort(v) => v.to_string(),
                Attribute::TailTarget(v) => v.to_string(),
                Attribute::TailTooltip(v) => v.to_string(),
                Attribute::TailUrl(v) => v.to_string(),
                Attribute::Target(v) => v.to_string(),
                Attribute::Tooltip(v) => v.to_string(),
                Attribute::TrueColor(v) => v.to_string(),
                Attribute::Url(v) => v.to_string(),
                Attribute::Vertices(v) => v
                    .iter()
                    .map(Point::to_string)
                    .collect::<Vec<String>>()
                    .join(" "),
                Attribute::ViewPort(v) => v.to_string(),
                Attribute::VoronoiMargin(v) => v.to_string(),
                Attribute::Weight(v) => v.to_string(),
                Attribute::Width(v) => v.to_string(),
                Attribute::XDotVersion(v) => v.to_string(),
                Attribute::ExternalLabel(v) => v.to_string(),
                Attribute::ExternalLabelPosition(v) => v.to_string(),
            }
        )
    }
}

impl Attribute {
    pub fn name(&self) -> &'static str {
        match self {
            Attribute::Area(_) => "area",
            Attribute::ArrowHead(_) => "arrowhead",
            Attribute::ArrowSize(_) => "arrowsize",
            Attribute::ArrowTail(_) => "arrowtail",
            Attribute::Background(_) => "_background",
            Attribute::BoundingBox(_) => "bb",
            Attribute::BackgroundColor(_) => "bgcolor",
            Attribute::Center(_) => "center",
            Attribute::Charset(_) => "charset",
            Attribute::Class(_) => "class",
            Attribute::ClusterRank(_) => "clusterrank",
            Attribute::Color(_) => "color",
            Attribute::ColorScheme(_) => "colorscheme",
            Attribute::Comment(_) => "comment",
            Attribute::Compound(_) => "compound",
            Attribute::Concentrate(_) => "concentrate",
            Attribute::Constraint(_) => "constraint",
            Attribute::Damping(_) => "damping",
            Attribute::Decorate(_) => "decorate",
            Attribute::DefaultDistance(_) => "defaultdist",
            Attribute::DimensionsLayout(_) => "dim",
            Attribute::DimensionsRender(_) => "dimen",
            Attribute::Direction(_) => "dir",
            Attribute::DirEdgeConstraints(_) => "diredgeconstraints",
            Attribute::Distortion(_) => "distortion",
            Attribute::Dpi(_) => "dpi",
            Attribute::EdgeUrl(_) => "edgeURL",
            Attribute::EdgeTarget(_) => "edgetarget",
            Attribute::EdgeTooltip(_) => "edgetooltip",
            Attribute::Epsilon(_) => "epsilon",
            Attribute::EdgeSeparation(_) => "esep",
            Attribute::FillColor(_) => "fillcolor",
            Attribute::FixedSize(_) => "fixedsize",
            Attribute::FontColor(_) => "fontcolor",
            Attribute::FontName(_) => "fontname",
            Attribute::FontNames(_) => "fontnames",
            Attribute::FontPath(_) => "fontpath",
            Attribute::FontSize(_) => "fontsize",
            Attribute::ForceLabels(_) => "forcelabels",
            Attribute::GradientAngle(_) => "gradientangle",
            Attribute::Group(_) => "group",
            Attribute::HeadLabelPosition(_) => "head_lp",
            Attribute::HeadClip(_) => "headclip",
            Attribute::HeadUrl(_) => "headURL",
            Attribute::HeadLabel(_) => "headlabel",
            Attribute::HeadPort(_) => "headport",
            Attribute::HeadTarget(_) => "headtarget",
            Attribute::HeadTooltip(_) => "headtooltip",
            Attribute::Height(_) => "height",
            Attribute::Id(_) => "id",
            Attribute::Image(_) => "image",
            Attribute::ImagePath(_) => "imagepath",
            Attribute::ImagePosition(_) => "imageposition",
            Attribute::ImageScale(_) => "imagescale",
            Attribute::InputScale(_) => "inputscale",
            Attribute::SpringConstant(_) => "K",
            Attribute::Label(_) => "label",
            Attribute::LabelScheme(_) => "label_scheme",
            Attribute::LabelAngle(_) => "labelangle",
            Attribute::LabelDistance(_) => "labeldistance",
            Attribute::LabelFloat(_) => "labelfloat",
            Attribute::LabelFontColor(_) => "labelfontcolor",
            Attribute::LabelFontName(_) => "labelfontname",
            Attribute::LabelFontSize(_) => "labelfontsize",
            Attribute::LabelJustification(_) => "labeljust",
            Attribute::LabelLocation(_) => "labelloc",
            Attribute::LabelTarget(_) => "labeltarget",
            Attribute::LabelTooltip(_) => "labeltooltip",
            Attribute::LabelUrl(_) => "labelURL",
            Attribute::Landscape(_) => "landscape",
            Attribute::Layer(_) => "layer",
            Attribute::Layers(_) => "layers",
            Attribute::LayerSelect(_) => "layerselect",
            Attribute::LayoutEngine(_) => "layout",
            Attribute::Length(_) => "len",
            Attribute::Levels(_) => "levels",
            Attribute::LevelsGap(_) => "levelsgap",
            Attribute::LabelHeight(_) => "lheight",
            Attribute::LabelPosition(_) => "lp",
            Attribute::LabelWidth(_) => "lwidth",
            Attribute::LogicalHead(_) => "lhead",
            Attribute::LogicalTail(_) => "ltail",
            Attribute::Margin(_) => "margin",
            Attribute::MaxIterations(_) => "maxiter",
            Attribute::McLimit(_) => "mclimit",
            Attribute::MinDistance(_) => "mindist",
            Attribute::MinLength(_) => "minlen",
            Attribute::Mode(_) => "mode",
            Attribute::Model(_) => "model",
            Attribute::Mosek(_) => "mosek",
            Attribute::NewRank(_) => "newrank",
            Attribute::NodeSeparation(_) => "nodesep",
            Attribute::NoJustification(_) => "nojustify",
            Attribute::Normalize(_) => "normalize",
            Attribute::NoTranslate(_) => "notranslate",
            Attribute::NetworkSimplexLimit(_) => "nslimit",
            Attribute::NetworkSimplexLimit1(_) => "nslimit1",
            Attribute::Ordering(_) => "ordering",
            Attribute::Orientation(_) => "orientation",
            Attribute::OutputOrder(_) => "outputorder",
            Attribute::Overlap(_) => "overlap",
            Attribute::OverlapScaling(_) => "overlap_scaling",
            Attribute::OverlapShrink(_) => "overlap_shrink",
            Attribute::Pack(_) => "pack",
            Attribute::PackMode(_) => "packmode",
            Attribute::Pad(_) => "pad",
            Attribute::PageSize(_) => "pagesize",
            Attribute::PageOutputDirection(_) => "pagedir",
            Attribute::PenColor(_) => "pencolor",
            Attribute::PenWidth(_) => "penwidth",
            Attribute::Peripheries(_) => "peripheries",
            Attribute::Pin(_) => "pin",
            Attribute::Position(_) => "pos",
            Attribute::QuadTree(_) => "quadtree",
            Attribute::Quantum(_) => "quantum",
            Attribute::Rank(_) => "rank",
            Attribute::RankDirection(_) => "rankdir",
            Attribute::RankSeparation(_) => "ranksep",
            Attribute::AspectRatio(_) => "ratio",
            Attribute::RecordRectangles(_) => "rects",
            Attribute::Regular(_) => "regular",
            Attribute::RerunCrossingMinimization(_) => "remincross",
            Attribute::RepulsiveForce(_) => "repulsiveforce",
            Attribute::Resolution(_) => "resolution",
            Attribute::Root(_) => "root",
            Attribute::RootRef(_) => "root",
            Attribute::Rotate(_) => "rotate",
            Attribute::Rotation(_) => "rotation",
            Attribute::SameHead(_) => "samehead",
            Attribute::SameTail(_) => "sametail",
            Attribute::SamplePoints(_) => "samplepoints",
            Attribute::Scale(_) => "scale",
            Attribute::SearchSize(_) => "searchsize",
            Attribute::Separation(_) => "sep",
            Attribute::Shape(_) => "shape",
            Attribute::ShowBoxes(_) => "showboxes",
            Attribute::Sides(_) => "sides",
            Attribute::Size(_) => "size",
            Attribute::Skew(_) => "skew",
            Attribute::Smoothing(_) => "smoothing",
            Attribute::SortValue(_) => "sortv",
            Attribute::Splines(_) => "splines",
            Attribute::Start(_) => "start",
            Attribute::Style(_) => "style",
            Attribute::Stylesheet(_) => "stylesheet",
            Attribute::TailLabelPosition(_) => "tail_lp",
            Attribute::TailClip(_) => "tailclip",
            Attribute::TailLabel(_) => "taillabel",
            Attribute::TailPort(_) => "tailport",
            Attribute::TailTarget(_) => "tailtarget",
            Attribute::TailTooltip(_) => "tailtooltip",
            Attribute::TailUrl(_) => "tailurl",
            Attribute::Target(_) => "target",
            Attribute::Tooltip(_) => "tooltip",
            Attribute::TrueColor(_) => "trucolor",
            Attribute::Url(_) => "URL",
            Attribute::Vertices(_) => "vertices",
            Attribute::ViewPort(_) => "viewport",
            Attribute::VoronoiMargin(_) => "voro_margin",
            Attribute::Weight(_) => "weight",
            Attribute::Width(_) => "width",
            Attribute::XDotVersion(_) => "xdotversion",
            Attribute::ExternalLabel(_) => "xlabel",
            Attribute::ExternalLabelPosition(_) => "xlp",
        }
    }
}

// ------------------------------------------------------------------------------------------------
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

impl Display for AspectRatio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Fill => "fill".to_string(),
                Self::Compress => "compress".to_string(),
                Self::Expand => "expand".to_string(),
                Self::Auto => "auto".to_string(),
                Self::Exactly(v) => v.to_string(),
            }
        )
    }
}

impl From<Double> for AspectRatio {
    fn from(v: Double) -> Self {
        Self::Exactly(v)
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for ClipSide {
    fn default() -> Self {
        Self::None
    }
}

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

// ------------------------------------------------------------------------------------------------

impl Default for ClusterRank {
    fn default() -> Self {
        Self::Local
    }
}

impl Display for ClusterRank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Local => "local",
                Self::Global => "global",
                Self::None => "none",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for ClusterStyles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Filled => "filled",
                Self::Striped => "striped",
                Self::Rounded => "rounded",
                Self::Radial => "radial",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Color::Rgb { r, g, b } => format!("#{:02x}{:02x}{:02x}", r, g, b),
                Color::Rgba { r, g, b, a } => format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a),
                Color::Hsv { h, s, v } => format!("{:.3} {:.3} {:.3}", h, s, v),
                Color::Name(v) => v.to_string(),
            }
        )
    }
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Rgb { r, g, b }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::Rgba { r, g, b, a }
    }

    pub fn hsv(h: Scale, s: Scale, v: Scale) -> Self {
        Self::Hsv { h, s, v }
    }

    pub fn named(s: &str) -> Result<Self, crate::error::Error> {
        if s.chars()
            .all(|c| c.is_ascii_alphabetic() || c.is_ascii_digit())
        {
            Ok(Self::Name(s.to_string()))
        } else {
            Err(invalid_value("Color", &s))
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for ColorOrList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Color(v) => v.to_string(),
                Self::List(vs) => vs
                    .iter()
                    .map(WeightedColor::to_string)
                    .collect::<Vec<String>>()
                    .join(":"),
            }
        )
    }
}

impl From<Color> for ColorOrList {
    fn from(v: Color) -> Self {
        Self::Color(v)
    }
}

impl From<Vec<WeightedColor>> for ColorOrList {
    fn from(vs: Vec<WeightedColor>) -> Self {
        Self::List(vs)
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for CompassPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::North => "n",
                Self::NorthEast => "ne",
                Self::East => "e",
                Self::SouthEast => "se",
                Self::South => "s",
                Self::SouthWest => "sw",
                Self::West => "w",
                Self::NorthWest => "nw",
                Self::Center => "c",
                Self::Appropriate => "_",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

double_newtype!(Degrees, "Degrees", is_valid_degrees);

// ------------------------------------------------------------------------------------------------

impl Default for Direction {
    fn default() -> Self {
        Self::None
    }
}

impl Display for Direction {
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

// ------------------------------------------------------------------------------------------------

impl Default for DistanceMatrix {
    fn default() -> Self {
        Self::Shortpath
    }
}

impl Display for DistanceMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Circuit => "circuit",
                Self::Shortpath => "shortpath",
                Self::Subset => "subset",
                Self::Mds => "mds",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for EdgeStyles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Dashed => "dashed",
                Self::Dotted => "dotted",
                Self::Solid => "solid",
                Self::Invisible => "invisible",
                Self::Bold => "bold",
                Self::Tapered => "tapered",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

string_newtype!(EscapedString, "EscapedString", is_valid_esc_string);

// ------------------------------------------------------------------------------------------------

impl Default for FontName {
    fn default() -> Self {
        Self::times_roman()
    }
}

impl Display for FontName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.to_string_inner(None))
    }
}

impl FontName {
    pub fn family(s: &str) -> Result<Self, crate::error::Error> {
        if Self::is_valid_family(s) {
            Ok(Self {
                family: s.to_string(),
                slant: None,
                weight: None,
                separator: String::from(" "),
            })
        } else {
            Err(invalid_value("FontName", &s))
        }
    }

    pub fn list(vs: Vec<FontName>) -> Self {
        Self {
            family: vs
                .iter()
                .map(|n| n.to_string_inner(Some(" ")))
                .collect::<Vec<String>>()
                .join(","),
            slant: None,
            weight: None,
            separator: String::from(" "),
        }
    }

    pub fn times_roman() -> Self {
        Self::family("Times-Roman").unwrap()
    }

    pub fn helvetica() -> Self {
        Self::family("Helvetica").unwrap()
    }

    pub fn courier() -> Self {
        Self::family("Courier").unwrap()
    }

    pub fn slant(self, v: FontSlant) -> Self {
        let mut self_mut = self;
        self_mut.slant = Some(v);
        self_mut
    }

    pub fn italic(self) -> Self {
        self.slant(FontSlant::Italic)
    }

    pub fn oblique(self) -> Self {
        self.slant(FontSlant::Oblique)
    }

    pub fn roman(self) -> Self {
        self.slant(FontSlant::Roman)
    }

    pub fn weight(self, v: FontWeight) -> Self {
        let mut self_mut = self;
        self_mut.weight = Some(v);
        self_mut
    }

    pub fn light(self) -> Self {
        self.weight(FontWeight::Light)
    }

    pub fn medium(self) -> Self {
        self.weight(FontWeight::Medium)
    }

    pub fn demi_bold(self) -> Self {
        self.weight(FontWeight::DemiBold)
    }

    pub fn bold(self) -> Self {
        self.weight(FontWeight::Bold)
    }

    pub fn black(self) -> Self {
        self.weight(FontWeight::Black)
    }

    pub fn is_valid_family(s: &str) -> bool {
        s.is_ascii()
    }

    pub fn to_string_inner(&self, separator: Option<&str>) -> String {
        let separator = if let Some(separator) = separator {
            separator
        } else {
            self.separator.as_str()
        };
        format!(
            "{}{}{}",
            self.family,
            self.slant
                .map(|v| format!("{}{}", separator, v))
                .unwrap_or_default(),
            self.weight
                .map(|v| format!("{}{}", separator, v))
                .unwrap_or_default()
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for FontNameMapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Svg => "svg",
                Self::Postscript => "ps",
                Self::FontConfig => "hd",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for FontSlant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Italic => "italic",
                Self::Oblique => "oblique",
                Self::Roman => "roman",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for FontWeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Light => "light",
                Self::Medium => "medium",
                Self::DemiBold => "demibold",
                Self::Bold => "bold",
                Self::Black => "black",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for GraphStyles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Filled => "filled",
                Self::Radial => "radial",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for ImagePosition {
    fn default() -> Self {
        Self::MiddleCentered
    }
}

impl Display for ImagePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::TopLeft => "tl",
                Self::TopCentered => "tc",
                Self::TopRight => "tr",
                Self::MiddleLeft => "ml",
                Self::MiddleCentered => "mc",
                Self::MiddleRight => "mr",
                Self::BottomLeft => "bl",
                Self::BottomCentered => "bc",
                Self::BottomRight => "br",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for LabelJustification {
    fn default() -> Self {
        Self::Centered
    }
}

impl Display for LabelJustification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Left => "l",
                Self::Centered => "c",
                Self::Right => "r",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for LabelLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Top => "t",
                Self::Centered => "c",
                Self::Bottom => "b",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl std::fmt::Display for LabelString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self
            .0
            .chars()
            .any(|c| c.is_whitespace() || c.is_control() || c == '"')
        {
            write!(f, "{:?}", self.0)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl AsRef<str> for LabelString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl FromStr for LabelString {
    type Err = crate::error::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if is_valid_label_string(value) {
            Ok(Self(value.to_string()))
        } else {
            Err(invalid_value("LabelString", &value))
        }
    }
}

impl LabelString {
    pub fn new_unchecked(s: &str) -> Self {
        Self(s.to_string())
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for LayerList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.layers.join(&self.separator))
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for LayerRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.layers
                .iter()
                .map(LayerRangeValue::to_string)
                .collect::<Vec<String>>()
                .join(&self.separator)
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for LayerRangeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Range(v) => v.to_string(),
                Self::Index(v) => v.to_string(),
                Self::All => "all".to_string(),
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for LayoutOptimization {
    fn default() -> Self {
        Self::StressMajorization
    }
}

impl Display for LayoutOptimization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::GradientDescent => "KK",
                Self::StressMajorization => "major",
                Self::StochasticGradientDescent => "sgd",
                Self::Hierarchical => "hier",
                Self::IpSeparation => "ipsep",
                Self::SpringElectrical => "spring",
                Self::MaxEnt => "maxent",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for NodeStyles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Dashed => "dashed",
                Self::Dotted => "dotted",
                Self::Solid => "solid",
                Self::Invisible => "invisible",
                Self::Bold => "bold",
                Self::Filled => "filled",
                Self::Striped => "striped",
                Self::Wedged => "wedged",
                Self::Diagonals => "diagonals",
                Self::Rounded => "rounded",
                Self::Radial => "radial",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for Ordering {
    fn default() -> Self {
        Self::None
    }
}

impl Display for Ordering {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::In => "in",
                Self::Out => "out",
                Self::None => "",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for OutputOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::BreadthFirst => "breadthfirst",
                Self::NodesFirst => "nodesfirst",
                Self::EdgesFirst => "edgesfirst",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for Overlap {
    fn default() -> Self {
        Self::True
    }
}

impl Display for Overlap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::True => "true".to_string(),
                Self::False => "false".to_string(),
                Self::Scale => "scale".to_string(),
                Self::Prism(v) => format!("prism{}", v),
                Self::Voronoi => "voronoi".to_string(),
                Self::ScaleXy => "scalexy".to_string(),
                Self::Compress => "compress".to_string(),
                Self::Quadratic => "vpsc".to_string(),
                Self::Ipsep => "ipsep".to_string(),
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for PackMode {
    fn default() -> Self {
        Self::Node
    }
}

impl Display for PackMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Node => "node".to_string(),
                Self::Graph => "graph".to_string(),
                Self::Cluster => "clust".to_string(),
                Self::Array {
                    order,
                    align,
                    count,
                } => format!(
                    "array_{}{}{}",
                    order.map(|v| v.to_string()).unwrap_or_default(),
                    align.map(|v| v.to_string()).unwrap_or_default(),
                    count.map(|v| v.to_string()).unwrap_or_default()
                ),
            }
        )
    }
}

impl PackMode {
    pub fn array(
        order: Option<PackModeOrder>,
        align: Option<PackModeAlign>,
        count: Option<Unsigned>,
    ) -> Self {
        Self::Array {
            order,
            align,
            count,
        }
    }
    pub fn array_with_alignment(align: PackModeAlign) -> Self {
        Self::Array {
            order: None,
            align: Some(align),
            count: None,
        }
    }

    pub fn array_row_count(count: Unsigned) -> Self {
        Self::Array {
            order: None,
            align: None,
            count: Some(count),
        }
    }

    pub fn array_row_count_with_alignment(count: Unsigned, align: PackModeAlign) -> Self {
        Self::Array {
            order: None,
            align: Some(align),
            count: Some(count),
        }
    }

    pub fn array_column_count(count: Unsigned) -> Self {
        Self::Array {
            order: Some(PackModeOrder::Column),
            align: None,
            count: Some(count),
        }
    }

    pub fn array_column_count_with_alignment(count: Unsigned, align: PackModeAlign) -> Self {
        Self::Array {
            order: Some(PackModeOrder::Column),
            align: Some(align),
            count: Some(count),
        }
    }

    pub fn array_user_count(count: Unsigned) -> Self {
        Self::Array {
            order: Some(PackModeOrder::User),
            align: None,
            count: Some(count),
        }
    }

    pub fn array_user_count_with_alignment(count: Unsigned, align: PackModeAlign) -> Self {
        Self::Array {
            order: Some(PackModeOrder::User),
            align: Some(align),
            count: Some(count),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for PackModeAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Top => "t",
                Self::Bottom => "b",
                Self::Left => "l",
                Self::Right => "r",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for PackModeOrder {
    fn default() -> Self {
        Self::Row
    }
}

impl Display for PackModeOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Row => "",
                Self::Column => "c",
                Self::User => "u",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for PageOutputDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.major, self.minor)
    }
}

impl PageOutputDirection {
    pub fn new(major: RankDirection, minor: RankDirection) -> Self {
        Self { major, minor }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.x,
            self.y,
            if self.fixed { "!" } else { "" }
        )
    }
}

impl Point {
    pub fn new(x: Double, y: Double) -> Self {
        Self { x, y, fixed: false }
    }

    pub fn new_fixed(x: Double, y: Double) -> Self {
        Self { x, y, fixed: true }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for PointOrBoth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Both(v) => v.to_string(),
                Self::Point(v) => v.to_string(),
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for PortPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.port_name,
            self.compass_point
                .map(|c| format!(":{}", c))
                .unwrap_or_default()
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Point(v) => v.to_string(),
                Self::SplineType(v) => v.to_string(),
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

double_newtype!(Positive, "Positive", is_valid_positive);

double_newtype!(
    PositiveNonZero,
    "PositiveNonZero",
    is_valid_positive_non_zero
);

// ------------------------------------------------------------------------------------------------

impl Default for QuadTreeType {
    fn default() -> Self {
        Self::Normal
    }
}

impl Display for QuadTreeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Normal => "notmal",
                Self::Fast => "fast",
                Self::None => "none",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for RankDirection {
    fn default() -> Self {
        Self::TopToBottom
    }
}

impl Display for RankDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::BottomToTop => "BT",
                Self::TopToBottom => "TB",
                Self::LeftToRight => "LR",
                Self::RightToLeft => "RL",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for RankSeparation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.min_vertical_distance
                .iter()
                .map(f64::to_string)
                .collect::<Vec<String>>()
                .join(":"),
            if self.equally { " equally" } else { "" }
        )
    }
}

impl RankSeparation {
    fn new_inner(min_vertical_distances: Vec<Double>, equally: bool) -> Self {
        const MIN: f64 = 0.02f64;
        Self {
            min_vertical_distance: min_vertical_distances
                .into_iter()
                .map(|v| if v < MIN { MIN } else { v })
                .collect(),
            equally,
        }
    }

    pub fn new(min_vertical_distances: Vec<Double>) -> Self {
        Self::new_inner(min_vertical_distances, false)
    }

    pub fn new_one(min_vertical_distance: Double) -> Self {
        Self::new_inner(vec![min_vertical_distance], false)
    }

    pub fn new_equally(min_vertical_distances: Vec<Double>) -> Self {
        Self::new_inner(min_vertical_distances, true)
    }

    pub fn new_one_equally(min_vertical_distance: Double) -> Self {
        Self::new_inner(vec![min_vertical_distance], true)
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for RankType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Max => "min",
                Self::Min => "max",
                Self::Same => "same",
                Self::Sink => "sink",
                Self::Source => "source",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\"{},{},{},{}\"",
            self.lower_left_x, self.lower_left_y, self.upper_right_x, self.upper_right_y
        )
    }
}

impl Rectangle {
    pub fn new(
        lower_left_x: Double,
        lower_left_y: Double,
        upper_right_x: Double,
        upper_right_y: Double,
    ) -> Self {
        Self {
            lower_left_x,
            lower_left_y,
            upper_right_x,
            upper_right_y,
        }
    }

    pub fn new_from(lower_left: Point, upper_right: Point) -> Self {
        Self::new(lower_left.x, lower_left.y, upper_right.x, upper_right.y)
    }
}

// ------------------------------------------------------------------------------------------------

double_newtype!(Scale, "Scale", is_valid_scale);

// ------------------------------------------------------------------------------------------------

impl Default for Shape {
    fn default() -> Self {
        Self::ellipse()
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Shape {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Shape {
    pub fn rectangle() -> Self {
        Self(String::from("box"))
    }
    pub fn polygon() -> Self {
        Self(String::from("polygon"))
    }
    pub fn ellipse() -> Self {
        Self(String::from("ellipse"))
    }
    pub fn oval() -> Self {
        Self(String::from("oval"))
    }
    pub fn circle() -> Self {
        Self(String::from("circle"))
    }
    pub fn point() -> Self {
        Self(String::from("point"))
    }
    pub fn egg() -> Self {
        Self(String::from("egg"))
    }
    pub fn triangle() -> Self {
        Self(String::from("triangle"))
    }
    pub fn plaintext() -> Self {
        Self(String::from("plaintext"))
    }
    pub fn none() -> Self {
        Self(String::from("plaintext"))
    }
    pub fn plain() -> Self {
        Self(String::from("plain"))
    }
    pub fn diamond() -> Self {
        Self(String::from("diamond"))
    }
    pub fn trapezium() -> Self {
        Self(String::from("trapezium"))
    }
    pub fn parallelogram() -> Self {
        Self(String::from("parallelogram"))
    }
    pub fn house() -> Self {
        Self(String::from("house"))
    }
    pub fn pentagon() -> Self {
        Self(String::from("pentagon"))
    }
    pub fn hexagon() -> Self {
        Self(String::from("hexagon"))
    }
    pub fn septagon() -> Self {
        Self(String::from("septagon"))
    }
    pub fn octagon() -> Self {
        Self(String::from("octagon"))
    }
    pub fn double_circle() -> Self {
        Self(String::from("doublecircle"))
    }
    pub fn double_octagon() -> Self {
        Self(String::from("doubleoctagon"))
    }
    pub fn triple_octagon() -> Self {
        Self(String::from("tripleoctagon"))
    }
    pub fn inverted_triangle() -> Self {
        Self(String::from("invtriangle"))
    }
    pub fn inverted_trapezium() -> Self {
        Self(String::from("invtrapezium"))
    }
    pub fn inverted_house() -> Self {
        Self(String::from("invhouse"))
    }
    pub fn m_diamond() -> Self {
        Self(String::from("Mdiamond"))
    }
    pub fn m_square() -> Self {
        Self(String::from("Msquare"))
    }
    pub fn m_circle() -> Self {
        Self(String::from("Mcircle"))
    }
    pub fn square() -> Self {
        Self(String::from("square"))
    }
    pub fn star() -> Self {
        Self(String::from("star"))
    }
    pub fn underline() -> Self {
        Self(String::from("underline"))
    }
    pub fn cylinder() -> Self {
        Self(String::from("cylinder"))
    }
    pub fn note() -> Self {
        Self(String::from("note"))
    }
    pub fn tab() -> Self {
        Self(String::from("tab"))
    }
    pub fn folder() -> Self {
        Self(String::from("folder"))
    }
    pub fn box3d() -> Self {
        Self(String::from("box3d"))
    }
    pub fn component() -> Self {
        Self(String::from("component"))
    }
    pub fn promoter() -> Self {
        Self(String::from("promoter"))
    }
    pub fn cds() -> Self {
        Self(String::from("cds"))
    }
    pub fn terminator() -> Self {
        Self(String::from("terminator"))
    }
    pub fn utr() -> Self {
        Self(String::from("utr"))
    }
    pub fn primersite() -> Self {
        Self(String::from("primersite"))
    }
    pub fn restrictionsite() -> Self {
        Self(String::from("restrictionsite"))
    }
    pub fn fivepoverhang() -> Self {
        Self(String::from("fivepoverhang"))
    }
    pub fn threepoverhang() -> Self {
        Self(String::from("threepoverhang"))
    }
    pub fn noverhang() -> Self {
        Self(String::from("noverhang"))
    }
    pub fn assembly() -> Self {
        Self(String::from("assembly"))
    }
    pub fn signature() -> Self {
        Self(String::from("signature"))
    }
    pub fn insulator() -> Self {
        Self(String::from("insulator"))
    }
    pub fn ribosite() -> Self {
        Self(String::from("ribosite"))
    }
    pub fn rnastab() -> Self {
        Self(String::from("rnastab"))
    }
    pub fn proteasesite() -> Self {
        Self(String::from("proteasesite"))
    }
    pub fn proteinstab() -> Self {
        Self(String::from("proteinstab"))
    }
    pub fn rpromoter() -> Self {
        Self(String::from("rpromoter"))
    }
    pub fn rarrow() -> Self {
        Self(String::from("rarrow"))
    }
    pub fn larrow() -> Self {
        Self(String::from("larrow"))
    }
    pub fn lpromoter() -> Self {
        Self(String::from("lpromoter"))
    }

    pub fn record() -> Self {
        Self(String::from("record"))
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for Smoothing {
    fn default() -> Self {
        Self::None
    }
}

impl Display for Smoothing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "none",
                Self::AverageDistance => "avg_dist",
                Self::GraphDistance => "graph_dist",
                Self::PowerDistance => "power_dist",
                Self::Rng => "rng",
                Self::Spring => "spring",
                Self::Triangle => "triangle",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Splines {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "none",
                Self::Line => "line",
                Self::Spline => "spline",
                Self::Polyline => "polyline",
                Self::Orthogonal => "ortho",
                Self::Curved => "curved",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for SplineType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut strings: Vec<String> = Default::default();
        if let Some(point) = &self.end {
            strings.push(point.to_string());
        }
        if let Some(point) = &self.start {
            strings.push(point.to_string());
        }
        strings.push(self.point.to_string());
        if !self.triples.is_empty() {
            strings.push(
                self.triples
                    .iter()
                    .map(Triple::to_string)
                    .collect::<Vec<String>>()
                    .join(" "),
            )
        }
        write!(f, " {}", strings.join(" "))
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Start {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.style.map(|v| v.to_string()).unwrap_or_default(),
            self.seed.map(|v| v.to_string()).unwrap_or_default()
        )
    }
}

impl Start {
    fn new(style: Option<StartStyle>, seed: Option<Unsigned>) -> Self {
        Self { style, seed }
    }

    pub fn styled(style: StartStyle) -> Self {
        Self::new(Some(style), None)
    }

    pub fn seeded(seed: Unsigned) -> Self {
        Self::new(None, Some(seed))
    }

    pub fn styled_with_seed(style: StartStyle, seed: Unsigned) -> Self {
        Self::new(Some(style), Some(seed))
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for StartStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Regular => "regular",
                Self::AtSelf => "self",
                Self::Random => "random",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Node(v) => v.to_string(),
                Self::Edge(v) => v.to_string(),
                Self::Cluster(v) => v.to_string(),
                Self::Graph(v) => v.to_string(),
            }
        )
    }
}

impl From<ClusterStyles> for Style {
    fn from(v: ClusterStyles) -> Self {
        Self::Cluster(v)
    }
}

impl From<EdgeStyles> for Style {
    fn from(v: EdgeStyles) -> Self {
        Self::Edge(v)
    }
}

impl From<GraphStyles> for Style {
    fn from(v: GraphStyles) -> Self {
        Self::Graph(v)
    }
}

impl From<NodeStyles> for Style {
    fn from(v: NodeStyles) -> Self {
        Self::Node(v)
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Triple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.p_1, self.p_2, self.p_3)
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for ViewPort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.width, self.height)?;
        if let Some(zoom) = &self.zoom {
            write!(f, ",{}", zoom)?;
        }
        if let Some(center) = &self.center {
            write!(f, ",{}", center)?;
        }
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for ViewPortCenter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Point(v) => v.to_string(),
                Self::Node(v) => v.to_string(),
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for WeightedColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(weight) = &self.weight {
            write!(f, "{};{}", self.color, weight)
        } else {
            write!(f, "{}", self.color)
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

const DOUBLE_ZERO: Double = 0.0f64;
const DOUBLE_ONE: Double = 1.0f64;
const DOUBLE_DEGREE_MAX: Double = 360.0f64;

#[inline]
fn is_valid_degrees(value: Double) -> bool {
    value >= DOUBLE_ZERO && value <= DOUBLE_DEGREE_MAX
}

#[inline]
fn is_valid_positive(value: Double) -> bool {
    value.is_sign_positive()
}

#[inline]
fn is_valid_positive_non_zero(value: Double) -> bool {
    value.is_sign_positive() && value != DOUBLE_ZERO
}

#[inline]
fn is_valid_scale(value: Double) -> bool {
    value >= DOUBLE_ZERO && value <= DOUBLE_ONE
}

#[inline]
fn is_valid_esc_string(_value: &str) -> bool {
    unimplemented!()
}

#[inline]
fn is_valid_label_string(_s: &str) -> bool {
    true
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

