use std::collections::VecDeque;

#[derive(Debug)]
struct Node<'a> {
    value: &'a str,
    parent: Option<usize>,
    children: Vec<usize>,
}

#[derive(Debug)]
struct Tree<'a> {
    nodes: Vec<Node<'a>>,
}

impl<'a> Tree<'a> {
    fn new() -> Self {
        Tree { nodes: Vec::new() }
    }

    // Add a root node to the tree
    fn add_root(&mut self, value: &'a str) -> usize {
        let new_node = Node {
            value,
            parent: None,
            children: Vec::new(),
        };
        self.nodes.push(new_node);
        self.nodes.len() - 1
    }

    // Add a child to a node at a given index
    fn add_child(&mut self, parent_index: usize, value: &'a str) -> Option<usize> {
        if parent_index >= self.nodes.len() {
            return None; // Invalid parent index
        }

        let new_node = Node {
            value,
            parent: Some(parent_index),
            children: Vec::new(),
        };
        self.nodes.push(new_node);
        let child_index = self.nodes.len() - 1;

        self.nodes[parent_index].children.push(child_index);

        Some(child_index)
    }

    fn dfs(&self, index: usize, level: usize) {
        if index >= self.nodes.len() {
            return;
        }

        println!("{}{}", " ".repeat(level * 4), self.nodes[index].value);

        for &child_index in &self.nodes[index].children {
            self.dfs(child_index, level + 1);
        }
    }
}

fn main() {
    let mut tree = Tree::new();
    let root = tree.add_root("root");
    if let Some(child1) = tree.add_child(root, "child1") {
        tree.add_child(child1, "child1.1");
        tree.add_child(child1, "child1.2");
    }
    if let Some(child2) = tree.add_child(root, "child2") {
        tree.add_child(child2, "child2.1");
    }

    println!("DFS Traversal:");
    tree.dfs(root, 0);
    tree.dfs(root, 0);
    tree.dfs(root, 0);
}