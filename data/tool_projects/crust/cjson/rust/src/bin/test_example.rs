use cJSON::cjson::{parse, CJson};

fn parse_and_callback(json: &CJson, prefix: &str) {
    println!(
        "{} => parse_and_callback:\n{}",
        prefix,
        json.print_formatted()
    );
}

#[test]
fn test() {
    // Create four objects: root, node1, node2, node3
    let mut node1 = CJson::create_object();
    let mut node2 = CJson::create_object();
    let mut node3 = CJson::create_object();
    let mut root = CJson::create_object();

    // Fill node1
    node1
        .add_item_to_object("node1_key1", CJson::create_bool(false))
        .unwrap();
    node1
        .add_item_to_object("node1_key2", CJson::create_bool(true))
        .unwrap();

    // Fill node2
    node2
        .add_item_to_object("node2_key1", CJson::create_string("node2_value1"))
        .unwrap();
    node2
        .add_item_to_object("node2_key2", CJson::create_string("node2_value2"))
        .unwrap();

    // Fill node3
    node3
        .add_item_to_object("node3_key1", CJson::create_number(1000.0))
        .unwrap();
    node3
        .add_item_to_object("node3_key2", CJson::create_number(2000.0))
        .unwrap();

    // Place node3 inside node1 (must do it before node1 moves into `root`)
    node1.add_item_to_object("node1_node3", node3).unwrap();

    // Now add node1 and node2 into the root object
    root.add_item_to_object("root_node1", node1).unwrap();
    root.add_item_to_object("root_node2", node2).unwrap();

    // This calls our function that in the C code was `parse_and_callback(root, "prefix")`.
    // In Rust we pass references rather than raw pointers.
    parse_and_callback(&root, "prefix");

    // Convert the root JSON object to a string (unformatted)
    let buf = root.print_unformatted();
    println!("json:\n{}\n", buf);

    // Re‐parse from that string, just as the C code does.
    // Any errors are handled via Rust’s Result type.
    let parse_node = parse(&buf, false).expect("Failed to parse JSON back from string.");

    // The original C code manually frees memory. In Rust, once `root` and
    // `parse_node` go out of scope, memory is automatically reclaimed.
    // So there is no explicit `cJSON_Delete` or `free`.
    println!("Re‐parsed object:\n{}", parse_node.print_formatted());
}
fn main() {
}
