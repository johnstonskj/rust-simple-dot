use simple_dot::attributes::{LabelString, NodeAttributes, Styled};
use simple_dot::node::field_vec_to_string;
use simple_dot::{node::Field, Identifier, Node};
use std::str::FromStr;

#[test]
fn test_simple_nodes() {
    assert_eq!(
        Node::new(Identifier::new_unchecked("a")).to_string(),
        String::from("a\n")
    );
    assert_eq!(
        Node::new(Identifier::new_unchecked("a"),)
            .set_attributes(NodeAttributes::default().label(LabelString::from_str("An A").unwrap()))
            .to_string(),
        String::from("a [ label = \"An A\" ]\n")
    );
}

#[test]
fn test_field_empty() {
    assert_eq!(Field::empty().to_string(), String::from(""));
}

#[test]
fn test_field_with_port() {
    assert_eq!(Field::port("id").to_string(), String::from("<id>"));
}

#[test]
fn test_field_with_text() {
    assert_eq!(Field::with_text("hello").to_string(), String::from("hello"));
}

#[test]
fn test_field_with_both() {
    assert_eq!(
        Field::port_with_text("id", "hello").to_string(),
        String::from("<id> hello")
    );
}

#[test]
fn test_doc_example() {
    assert_eq!(
        field_vec_to_string(&vec![
            Field::with_text("hello&#92;nworld"),
            Field::Flip(vec![
                Field::with_text("b"),
                Field::Flip(vec![
                    Field::with_text("c"),
                    Field::port_with_text("here", "d"),
                    Field::with_text("e"),
                ]),
                Field::with_text("f"),
            ]),
            Field::with_text("g"),
            Field::with_text("h"),
        ]),
        String::from("hello&#92;nworld | { b | { c | <here> d | e } | f } | g | h")
    );
}
