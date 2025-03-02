#![allow(unused)]

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct EBNFParser;

fn main() {
    let csv = std::fs::read_to_string("test/bnf.ebnf").unwrap();

    let parse = EBNFParser::parse(Rule::syntax, &csv)
        .unwrap()
        .next()
        .unwrap()
        .into_inner();

    for p in parse {
        dbg!(p.as_str());
        // dbg!(p);
    }
    println!("Hello, world!");
}
