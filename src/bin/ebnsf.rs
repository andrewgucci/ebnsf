use ebnsf::parse_ebnf;

use std::path::PathBuf;

fn main() {
    let ebnf = std::fs::read_to_string("test/bnf.ebnf").unwrap();

    let diagram = parse_ebnf(&ebnf).unwrap();

    let output = PathBuf::from("test.svg");

    std::fs::write(&output, diagram.to_string().into_bytes()).unwrap();
}
