use tree_sitter::{Parser, Language};

fn main() {
    let mut parser = get_parser();
    let source_code = std::fs::read_to_string("/home/dan/documents/apps/temp/tree-sitter-rs/src/main.rs").unwrap();

    let tree = parser.parse(source_code.clone(), None).unwrap();
    let root_node = tree.root_node();
    let num_children = root_node.child_count();
    let source_code_vec: Vec<String> = source_code.lines().map(|l| l.to_string()).collect();

    for i in 0..num_children {
        let node = root_node.child(i).unwrap();
        let node_kind = node.kind();
        let start = node.start_position().row;
        let end = node.end_position().row;

        if node_kind == "function_item" {
            let source_rows = source_code_vec
                .iter()
                .skip(start)
                .take(end - start + 1)
                .map(|l| l.trim().to_string())
                .collect::<String>();

            println!("function: {} \n\n", source_rows);
        }
    }
}

fn get_parser() -> Parser {
    let mut parser = Parser::new();
    extern "C" { fn tree_sitter_rust() -> Language; }
    let language = unsafe { tree_sitter_rust() };
    parser.set_language(language).unwrap();
    return parser;
}
