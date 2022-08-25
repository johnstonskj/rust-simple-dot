# Crate simple_dot

Simple API for creating GraphViz [DOT language](https://graphviz.org/doc/info/lang.html) files.

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.34-green.svg)
[![crates.io](https://img.shields.io/crates/v/simple_dot.svg)](https://crates.io/crates/simple_dot)
[![docs.rs](https://docs.rs/simple_dot/badge.svg)](https://docs.rs/simple_dot)
![Build](https://github.com/johnstonskj/rust-simple_dot/workflows/Rust/badge.svg)
![Audit](https://github.com/johnstonskj/rust-simple_dot/workflows/Security%20audit/badge.svg)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-simple_dot.svg)](https://github.com/johnstonskj/rust-simple_dot/stargazers)

TBD

The goal of this crate is to provide an ergonomic interface in a builder style
to make creation of DOT files easy. This sacrifices performance and certainly
memory usage is higher than a crate like [dot](https://crates.io/crates/dot).

## Example 

``` rust
use crate::writer::write_graph;
use crate::graph::{Cluster, Graph, RootGraph};
use crate::node::Node;
use crate::style::{
    ClusterAttributes, ClusterStyles, Color, EdgeAttributes, FontName, GraphAttributes,
    LabelString, NodeAttributes, NodeStyles, Shape, Styled,
};
use crate::writer::Indenter;
use crate::Identifier;
use std::str::FromStr;

let fonts = FontName::list(vec![
    FontName::family("Helvetica").unwrap(),
    FontName::family("Arial").unwrap(),
    FontName::family("sans-serif").unwrap(),
]);
let root = RootGraph::new(Identifier::from_str("G").unwrap(), false, true)
    .set_attributes(GraphAttributes::default().font_name(fonts.clone()))
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
        Identifier::from_str("start").unwrap(),
        Identifier::from_str("a0").unwrap(),
    )
    .add_edge_between(
        Identifier::from_str("start").unwrap(),
        Identifier::from_str("b0").unwrap(),
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
    );

write_graph(&root, &mut std::io::stdout()).unwrap();
```

``` graphviz-(dot)
digraph G {
  fontname = "Helvetica,Arial,sans-serif";

  node [
    fontname = "Helvetica,Arial,sans-serif";
  ];

  edge [
    fontname = "Helvetica,Arial,sans-serif";
  ];

  subgraph cluster_0 {
    label = "process #1";
    color = lightgrey;
    style = filled;

    node [
      color = white;
      style = filled;
    ];

    a0 -> a1;
    a1 -> a2;
    a2 -> a3;
  }
  subgraph cluster_1 {
    label = "process #2";
    color = blue;

    node [
      style = filled;
    ];

    b0 -> b1;
    b1 -> b2;
    b2 -> b3;
  }

  start [
    shape = Mdiamond;
  ];
  end [
    shape = Msquare;
  ];

  start -> a0;
  start -> b0;
  a1 -> b3;
  b2 -> a3;
  a3 -> a0;
  a3 -> end;
  b3 -> end;
}
```

## Changes


**Version 0.1.0**

* Initial version.
