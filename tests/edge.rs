use simple_dot::{
    attributes::{EdgeAttributes, LabelString, Styled},
    Edge, Identifier,
};

#[test]
fn test_simple_edges() {
    let a = Identifier::new_unchecked("a");
    let b = Identifier::new_unchecked("b");

    assert_eq!(
        Edge::new(a, b)
            .set_attributes(EdgeAttributes::default().label(LabelString::new_unchecked("a to b")))
            .to_string(),
        String::from("a -- b [ label = \"a to b\" ]\n")
    );
}
