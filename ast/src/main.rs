use serde_json::Value;
use std::fs;

pub mod types;

use types::{
    ast::Ast,
    node::{ContractDefintion, FunctionDefinition, Node, NodeType},
};

#[derive(Debug)]
pub struct Function {
    pub name: String

}

#[derive(Debug)]
pub struct Contract {
    pub name: String,
    pub functions: Vec<Function>
}

fn collect(node: &Node) {
    let c: ContractDefintion = (&node.data).into();
    let mut contract = Contract {name: c.name, functions: vec![]};

    for node in node.nodes.iter() {
        if node.node_type == NodeType::FunctionDefinition {
            let f: FunctionDefinition = (&node.data).into();
            println!("FUNC {:#?}", f);
            contract.functions.push(Function {name: f.name});

        }
    }

    println!("HERE {:#?}", contract);
}

fn bfs(node: &Node, node_type: NodeType, f: fn(&Node)) {
    let mut q = vec![node];

    while let Some(node) = q.pop() {
        for node in node.nodes.iter() {
            if node.node_type == node_type {
                f(&node);
            }
            q.push(node);
        }
    }
}

fn main() {
    let file_path = "tmp/ERC20.json";
    let content = fs::read_to_string(file_path).unwrap();
    let ast = serde_json::from_str::<Ast>(&content).unwrap();
    // println!("{:#?}", ast);

    bfs(&ast.ast, NodeType::ContractDefinition, collect);
}
