#![allow(unused_variables)]
#![allow(dead_code)]

use serde_json;
use std::env;

mod rinha;

use crate::rinha::*;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    println!("Usage: {} <path_to_json_file>", args[0]);
    return;
  }

  // read file
  let json = std::fs::read_to_string("./example.json").unwrap();

  // parses it as JSON
  let parsed_json: serde_json::Value = serde_json::from_str(&json).unwrap();

  // converts to Rinha File
  let input = json_to_file(&parsed_json).unwrap().expr;
  println!("# Rinha Input:\n{}\n", term_to_string(&input));

  // converts to an HVM2 net
  let mut net = rinha_to_net(&input);
  println!("# HVM Net:\n{}", hvm_core::lang::show_net(&net));

  // reduces net to normal form
  net.normal(&hvm_core::core::Book::new());

  // converts back to a Rinha term
  let output = net_to_rinha(&net);
  println!("# Rinha Output:\n{}\n", term_to_string(&output));

  // shows stats
  println!("# HVM Stats:");
  println!("- rwts: {}", net.rwts);
  println!("- dref: {}", net.dref);
}
