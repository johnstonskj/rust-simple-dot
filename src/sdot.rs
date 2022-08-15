/*!
One-line description.

More detailed description, with

# Example

 */

use std::collections::HashMap;
use std::fmt::Display;
use unique_id::string::StringGenerator;
use unique_id::Generator;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Graph {
    id: String,
    label: Option<String>,
    options: Option<GraphOptions>,
    style: Option<GraphStyle>,
    default_node_style: Option<NodeStyle>,
    default_edge_style: Option<EdgeStyle>,
    nodes: HashMap<String, Node>,
    edges: Vec<Edge>,
    sub_graphs: Vec<SubGraph>,
}

#[derive(Clone, Debug)]
pub struct RootGraph {
    strict: bool,
    directed: bool,
    inner: Graph,
}

#[derive(Clone, Debug)]
pub struct GraphOptions {}

#[derive(Clone, Debug)]
pub struct GraphStyle {}

#[derive(Clone, Debug)]
pub struct SubGraph {
    cluster: bool,
    inner: Graph,
}

#[derive(Clone, Debug)]
pub struct Node {
    id: String,
    label: Option<String>,
    style: Option<NodeStyle>,
}

#[derive(Clone, Debug)]
pub struct NodeStyle {}

#[derive(Clone, Debug)]
pub struct Edge {
    source: String,
    target: String,
    label: Option<String>,
    style: Option<EdgeStyle>,
}

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

impl Default for Graph {
    fn default() -> Self {
        Self {
            id: StringGenerator::default().next_id(),
            label: Default::default(),
            options: Default::default(),
            style: Default::default(),
            default_node_style: Default::default(),
            default_edge_style: Default::default(),
            nodes: Default::default(),
            edges: Default::default(),
            sub_graphs: Default::default(),
        }
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(label) = &self.label {
            writeln!(f, "  label={:?}", label)?;
        }
        if let Some(options) = &self.options {
            writeln!(f, "{}", options)?;
        }
        if let Some(style) = &self.style {
            writeln!(f, "{}", style)?;
        }
        if let Some(default_node_style) = &self.default_node_style {
            writeln!(f, "{}", default_node_style)?;
        }
        if let Some(default_edge_style) = &self.default_edge_style {
            writeln!(f, "{}", default_edge_style)?;
        }
        for node in self.nodes.values() {
            writeln!(f, "{}", node)?;
        }
        for edge in &self.edges {
            writeln!(f, "{}", edge)?;
        }
        for sub_graph in &self.sub_graphs {
            writeln!(f, "{}", sub_graph)?;
        }
        Ok(())
    }
}

impl Graph {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_owned(),
            ..Default::default()
        }
    }

    pub fn labeled(label: &str) -> Self {
        Self {
            id: StringGenerator::default().next_id(),
            label: Some(label.to_owned()),
            ..Default::default()
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn set_id(&mut self, id: &str) -> &mut Self {
        self.id = id.to_owned();
        self
    }

    pub fn label(&self) -> Option<&String> {
        self.label.as_ref()
    }

    pub fn set_label(&mut self, label: &str) -> &mut Self {
        self.label = Some(label.to_owned());
        self
    }

    pub fn unset_label(&mut self) -> &mut Self {
        self.label = None;
        self
    }

    pub fn style(&self) -> Option<&GraphStyle> {
        self.style.as_ref()
    }

    pub fn set_style(&mut self, style: GraphStyle) -> &mut Self {
        self.style = Some(style);
        self
    }

    pub fn unset_style(&mut self) -> &mut Self {
        self.style = None;
        self
    }

    pub fn options(&self) -> Option<&GraphOptions> {
        self.options.as_ref()
    }

    pub fn set_options(&mut self, options: GraphOptions) -> &mut Self {
        self.options = Some(options);
        self
    }

    pub fn unset_options(&mut self) -> &mut Self {
        self.options = None;
        self
    }

    pub fn default_node_style(&self) -> Option<&NodeStyle> {
        self.default_node_style.as_ref()
    }

    pub fn set_default_node_style(&mut self, default_node_style: NodeStyle) -> &mut Self {
        self.default_node_style = Some(default_node_style);
        self
    }

    pub fn unset_default_node_style(&mut self) -> &mut Self {
        self.default_node_style = None;
        self
    }

    pub fn default_edge_style(&self) -> Option<&EdgeStyle> {
        self.default_edge_style.as_ref()
    }

    pub fn set_default_edge_style(&mut self, default_edge_style: EdgeStyle) -> &mut Self {
        self.default_edge_style = Some(default_edge_style);
        self
    }

    pub fn unset_default_edge_style(&mut self) -> &mut Self {
        self.default_edge_style = None;
        self
    }

    pub fn node(&self, id: &str) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.values()
    }

    pub fn add_node(&mut self, node: Node) {
        let _ = self.nodes.insert(node.id().to_owned(), node);
    }

    pub fn edges(&self) -> impl Iterator<Item = &Edge> {
        self.edges.iter()
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge)
    }

    pub fn sub_graphs(&self) -> impl Iterator<Item = &SubGraph> {
        self.sub_graphs.iter()
    }

    pub fn add_sub_graph(&mut self, sub_graph: SubGraph) {
        self.sub_graphs.push(sub_graph)
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for RootGraph {
    fn default() -> Self {
        Self {
            strict: Default::default(),
            directed: Default::default(),
            inner: Default::default(),
        }
    }
}

impl From<Graph> for RootGraph {
    fn from(v: Graph) -> Self {
        Self {
            strict: Default::default(),
            directed: Default::default(),
            inner: v,
        }
    }
}

impl Display for RootGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_strict() {
            write!(f, "strict ")?;
        }
        if self.is_directed() {
            write!(f, "di")?;
        }
        writeln!(f, "graph {} {{", self.inner.id())?;
        writeln!(f, "{}", self.inner)?;
        write!(f, "}}")
    }
}

impl AsRef<Graph> for RootGraph {
    fn as_ref(&self) -> &Graph {
        &self.inner
    }
}

impl AsMut<Graph> for RootGraph {
    fn as_mut(&mut self) -> &mut Graph {
        &mut self.inner
    }
}

impl From<RootGraph> for Graph {
    fn from(v: RootGraph) -> Self {
        v.inner
    }
}

impl RootGraph {
    pub fn digraph() -> Self {
        Self {
            strict: Default::default(),
            directed: Default::default(),
            inner: Default::default(),
        }
    }

    pub fn new(id: &str) -> Self {
        Self {
            strict: Default::default(),
            directed: Default::default(),
            inner: Graph::new(id),
        }
    }

    pub fn labeled(label: &str) -> Self {
        Self {
            strict: Default::default(),
            directed: Default::default(),
            inner: Graph::labeled(label),
        }
    }

    pub fn new_directed(id: &str) -> Self {
        Self {
            strict: Default::default(),
            directed: true,
            inner: Graph::new(id),
        }
    }

    pub fn directed_labeled(label: &str) -> Self {
        Self {
            strict: Default::default(),
            directed: true,
            inner: Graph::labeled(label),
        }
    }

    pub fn is_strict(&self) -> bool {
        self.strict
    }

    pub fn set_strict(&mut self, strict: bool) -> &mut Self {
        self.strict = strict;
        self
    }

    pub fn is_directed(&self) -> bool {
        self.directed
    }

    pub fn set_directed(&mut self, directed: bool) -> &mut Self {
        self.directed = directed;
        self
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for SubGraph {
    fn default() -> Self {
        Self {
            cluster: Default::default(),
            inner: Default::default(),
        }
    }
}

impl From<Graph> for SubGraph {
    fn from(v: Graph) -> Self {
        Self {
            cluster: Default::default(),
            inner: v,
        }
    }
}

impl Display for SubGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "subgraph {}{} {{",
            if self.is_cluster() { "cluster_" } else { "" },
            self.inner.id()
        )?;
        writeln!(f, "{}", self.inner)?;
        writeln!(f, "}}")
    }
}

impl AsRef<Graph> for SubGraph {
    fn as_ref(&self) -> &Graph {
        &self.inner
    }
}

impl AsMut<Graph> for SubGraph {
    fn as_mut(&mut self) -> &mut Graph {
        &mut self.inner
    }
}

impl From<SubGraph> for Graph {
    fn from(v: SubGraph) -> Self {
        v.inner
    }
}

impl SubGraph {
    pub fn cluster() -> Self {
        Self {
            cluster: Default::default(),
            inner: Default::default(),
        }
    }

    pub fn new(id: &str) -> Self {
        Self {
            cluster: Default::default(),
            inner: Graph::new(id),
        }
    }

    pub fn labeled(label: &str) -> Self {
        Self {
            cluster: Default::default(),
            inner: Graph::labeled(label),
        }
    }

    pub fn new_cluster(id: &str) -> Self {
        Self {
            cluster: true,
            inner: Graph::new(id),
        }
    }

    pub fn cluster_labeled(label: &str) -> Self {
        Self {
            cluster: true,
            inner: Graph::labeled(label),
        }
    }

    pub fn is_cluster(&self) -> bool {
        self.cluster
    }

    pub fn set_cluster(&mut self, cluster: bool) -> &mut Self {
        self.cluster = cluster;
        self
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for GraphOptions {
    fn default() -> Self {
        Self {}
    }
}

impl Display for GraphOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "graph [")?;
        writeln!(f, "]")
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for GraphStyle {
    fn default() -> Self {
        Self {}
    }
}

impl Display for GraphStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "graph [")?;
        writeln!(f, "]")
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for Node {
    fn default() -> Self {
        Self {
            id: StringGenerator::default().next_id(),
            label: Default::default(),
            style: Default::default(),
        }
    }
}

impl Display for Node {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Node {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            ..Default::default()
        }
    }

    pub fn labeled(label: &str) -> Self {
        Self {
            id: StringGenerator::default().next_id(),
            label: Some(label.to_string()),
            ..Default::default()
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn set_id(&mut self, id: &str) -> &mut Self {
        self.id = id.to_owned();
        self
    }

    pub fn label(&self) -> Option<&String> {
        self.label.as_ref()
    }

    pub fn set_label(&mut self, label: &str) -> &mut Self {
        self.label = Some(label.to_owned());
        self
    }

    pub fn unset_label(&mut self) -> &mut Self {
        self.label = None;
        self
    }

    pub fn style(&self) -> Option<&NodeStyle> {
        self.style.as_ref()
    }

    pub fn set_style(&mut self, style: NodeStyle) -> &mut Self {
        self.style = Some(style);
        self
    }

    pub fn unset_style(&mut self) -> &mut Self {
        self.style = None;
        self
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for NodeStyle {
    fn default() -> Self {
        Self {}
    }
}

impl Display for NodeStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "node [")?;
        writeln!(f, "]")
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Edge {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Edge {
    pub fn new(source: &str, target: &str) -> Self {
        Self {
            source: source.to_string(),
            target: target.to_string(),
            label: Default::default(),
            style: Default::default(),
        }
    }

    pub fn new_from(source: &Node, target: &Node) -> Self {
        Self {
            source: source.id().to_string(),
            target: target.id().to_string(),
            label: Default::default(),
            style: Default::default(),
        }
    }

    pub fn labeled(source: &str, target: &str, label: &str) -> Self {
        Self {
            source: source.to_string(),
            target: target.to_string(),
            label: Some(label.to_string()),
            style: Default::default(),
        }
    }

    pub fn labeled_from(source: &Node, target: &Node, label: &str) -> Self {
        Self {
            source: source.id().to_string(),
            target: target.id().to_string(),
            label: Some(label.to_string()),
            style: Default::default(),
        }
    }

    pub fn source(&self) -> &String {
        &self.source
    }

    pub fn set_source(&mut self, source: &str) -> &mut Self {
        self.source = source.to_owned();
        self
    }

    pub fn target(&self) -> &String {
        &self.target
    }

    pub fn set_target(&mut self, target: &str) -> &mut Self {
        self.target = target.to_owned();
        self
    }

    pub fn label(&self) -> Option<&String> {
        self.label.as_ref()
    }

    pub fn set_label(&mut self, label: &str) -> &mut Self {
        self.label = Some(label.to_owned());
        self
    }

    pub fn unset_label(&mut self) -> &mut Self {
        self.label = None;
        self
    }

    pub fn style(&self) -> Option<&EdgeStyle> {
        self.style.as_ref()
    }

    pub fn set_style(&mut self, style: EdgeStyle) -> &mut Self {
        self.style = Some(style);
        self
    }

    pub fn unset_style(&mut self) -> &mut Self {
        self.style = None;
        self
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

impl Display for EdgeStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "edge [")?;
        writeln!(f, "]")
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
    fn test_cluster_example() {
        let mut root = RootGraph::new_directed("G");
        let root_graph = root.as_mut();

        let mut cluster = SubGraph::cluster_labeled("process #1");
        let cluster_graph = cluster.as_mut();
        cluster_graph.set_id("0");
        root_graph.add_sub_graph(cluster);

        let mut cluster = SubGraph::cluster_labeled("process #2");
        let cluster_graph = cluster.as_mut();
        cluster_graph.set_id("1");
        root_graph.add_sub_graph(cluster.clone());

        println!("{}", root);
    }
}
