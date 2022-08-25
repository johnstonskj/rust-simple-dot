use simple_dot::attributes::{
    ClusterAttributes, ClusterStyles, Color, EdgeAttributes, FontName, GraphAttributes,
    GraphStyles, LabelString, NodeAttributes, NodeStyles, Shape, Styled,
};
use simple_dot::graph::{Cluster, Graph, RootGraph};
use simple_dot::node::Node;
use simple_dot::Identifier;
use std::str::FromStr;

pub fn gallery_cluster_example() -> RootGraph {
    let fonts = FontName::list(vec![
        FontName::family("Helvetica").unwrap(),
        FontName::family("Arial").unwrap(),
        FontName::family("sans-serif").unwrap(),
    ]);
    RootGraph::new(Identifier::from_str("G").unwrap(), false, true)
        .set_attributes(
            GraphAttributes::default()
                .font_name(fonts.clone())
                .style(vec![GraphStyles::Filled]),
        )
        .set_default_node_attributes(NodeAttributes::default().font_name(fonts.clone()))
        .set_default_edge_attributes(EdgeAttributes::default().font_name(fonts.clone()))
        .add_sub_graph(
            Cluster::new(0i64.into())
                .set_attributes(
                    ClusterAttributes::default()
                        .label(LabelString::from_str("process #1").unwrap())
                        .color(Color::named("lightgrey").unwrap().into())
                        .style(vec![ClusterStyles::Filled]),
                )
                .set_default_node_attributes(
                    NodeAttributes::default()
                        .color(Color::named("white").unwrap().into())
                        .style(vec![NodeStyles::Filled]),
                )
                .chain(vec![
                    Node::new(Identifier::from_str("a0").unwrap()),
                    Node::new(Identifier::from_str("a1").unwrap()),
                    Node::new(Identifier::from_str("a2").unwrap()),
                    Node::new(Identifier::from_str("a3").unwrap()),
                ]),
        )
        .add_sub_graph(
            Cluster::new(1i64.into())
                .set_attributes(
                    ClusterAttributes::default()
                        .label(LabelString::from_str("process #2").unwrap())
                        .color(Color::named("blue").unwrap().into()),
                )
                .set_default_node_attributes(
                    NodeAttributes::default().style(vec![NodeStyles::Filled]),
                )
                .chain(vec![
                    Node::new(Identifier::from_str("b0").unwrap()),
                    Node::new(Identifier::from_str("b1").unwrap()),
                    Node::new(Identifier::from_str("b2").unwrap()),
                    Node::new(Identifier::from_str("b3").unwrap()),
                ]),
        )
        .add_node(
            Node::new(Identifier::from_str("start").unwrap())
                .set_attributes(NodeAttributes::default().shape(Shape::m_diamond())),
        )
        .add_node(
            Node::new(Identifier::from_str("end").unwrap())
                .set_attributes(NodeAttributes::default().shape(Shape::m_square())),
        )
        .add_edge_between(
            Identifier::from_str("a1").unwrap(),
            Identifier::from_str("b3").unwrap(),
        )
        .add_edge_between(
            Identifier::from_str("b2").unwrap(),
            Identifier::from_str("a3").unwrap(),
        )
        .add_edge_between(
            Identifier::from_str("b2").unwrap(),
            Identifier::from_str("a3").unwrap(),
        )
        .add_edge_between(
            Identifier::from_str("a3").unwrap(),
            Identifier::from_str("a0").unwrap(),
        )
        .add_edge_between(
            Identifier::from_str("a3").unwrap(),
            Identifier::from_str("end").unwrap(),
        )
        .add_edge_between(
            Identifier::from_str("b3").unwrap(),
            Identifier::from_str("end").unwrap(),
        )
}
