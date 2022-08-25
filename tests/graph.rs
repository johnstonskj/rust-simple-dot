use simple_dot::{attributes::Styled, graph::Graph};

pub mod common;

#[test]
fn test_cluster_example() {
    let root = common::gallery_cluster_example();

    assert!(root.has_attributes());

    assert!(root.has_nodes());
    assert!(root.has_edges());
    assert!(root.has_sub_graphs());
}
