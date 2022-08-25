use simple_dot::attributes::{ArrowType, EdgeAttributes, FontName, GraphAttributes};

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
    assert_eq!(EdgeAttributes::default().to_string(), String::from("[]"));
}

#[test]
fn test_graph_style() {
    assert_eq!(
        GraphAttributes::default()
            .font_name(FontName::courier().bold())
            .to_string(),
        String::from("[ fontname = \"Courier bold\" ]")
    );
}

#[test]
fn test_edge_style() {
    // assert_eq!(
    //     EdgeStyle::default()
    //         .set_arrow_head(ArrowType::a_box())
    //         .set_arrow_tail(ArrowType::a_box())
    //         .to_string(),
    //     String::from("arrowhead = box; arrowtail = box; ")
    // );
}
