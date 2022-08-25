use simple_dot::writer::{write_graph, Indenter};

pub mod common;

#[test]
fn test_cluster_example() {
    let root = common::gallery_cluster_example();

    write_graph(&root, &mut std::io::stdout()).unwrap();
}

#[test]
fn test_indenter() {
    let indent = Indenter::pad_with_two_spaces();
    println!("{:#?}", indent);

    assert_eq!(indent.pad_string(), String::new());

    indent.indent();
    assert_eq!(indent.pad_string(), String::from("  "));

    indent.indent_by(2);
    assert_eq!(indent.pad_string(), String::from("      "));

    indent.unindent();
    assert_eq!(indent.pad_string(), String::from("    "));
}
