use std::fs;
use std::env;
use std::io::Read;
// use std::collections::HashSet;
// use std::iter::FromIterator;

#[derive(Clone)]
struct Node {
    parent: String,
    weight: u32,
    children: Vec<String>,
}

fn main() {

    let contents: String = open_file_read_contents();

    let forest: Vec<Node> = contents.lines().map(|line| {
        Node {
            parent: parse_node_parent(line),
            weight: parse_node_weight(line),
            children: parse_node_children(line),
        }
    }).collect();

    let tree_root: String = find_tree_root(&forest);
    println!("TREE ROOT: {:?}", &tree_root);

}

fn find_tree_root(tree_vec: &Vec<Node>) -> String {
    let root_const: String = String::from("ROOT");
    let mut start: String = match tree_vec.iter().cloned().nth(0) {
        Some(n) => n.parent,
        None => panic!("FAILURE : EMPTY FOREST"),
    };

    loop {
        let next_node: String = match tree_vec.iter().cloned()
            .filter(|n| n.children.contains(&start))
            .map(|n| n.parent).next() {
                Some(p) => p,
                None => root_const.clone(),
        };

        if next_node != root_const {
            start = next_node;
        } else {
            break start;
        }
    }
}

fn parse_node_parent(line: &str) -> String {
    match line.split_whitespace().nth(0) {
        Some(p) => String::from(p),
        None => panic!("FAILURE : PARSE NODE > PARENT"),
    }
}

fn parse_node_weight(line: &str) -> u32 {
    match line.split_whitespace().nth(1) {
        Some(p) => p.trim_matches(|c| c == '(' || c == ')').parse().unwrap(),
        None => panic!("FAILURE : PARSE NODE > WEIGHT"),
    }
}

fn parse_node_children(line: &str) -> Vec<String> {
    match line.split("->").nth(1) {
        Some(p) => p.split(", ").map(|x| String::from(x.trim())).collect(),
        None => Vec::new(),
    }
}

fn open_file_read_contents() -> String {
    let path = match env::args().nth(1) {
        Some(p) => p,
        None => panic!("FAILURE : FILE PATH"),
    };

    let mut file = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => panic!("FAILURE : OPEN FILE"),
    };

    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Ok(s) => s,
        Err(_) => panic!("FAILURE : READ FILE"),
    };
    buffer
}
