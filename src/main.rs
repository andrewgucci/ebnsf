// #![allow(unused, clippy::all)]

use std::path::PathBuf;

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use railroad as rr;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct EBNFParser;

fn main() {
    let csv = std::fs::read_to_string("test/bnf.ebnf").unwrap();

    let mut result = EBNFParser::parse(Rule::syntax, &csv).unwrap();
    let trees = result.next().expect("expected root expr").into_inner();

    let nodes = trees
        .map(|p| {
            Box::new(rr::Sequence::new(vec![
                Box::new(rr::SimpleStart) as Box<dyn rr::Node>,
                Box::new(make_node(p)),
                Box::new(rr::SimpleStart),
            ]))
        })
        .collect::<Vec<_>>();

    let mut diagram = rr::Diagram::new(Box::new(rr::VerticalGrid::new(nodes)));

    diagram.add_css(rr::DEFAULT_CSS);

    let output = PathBuf::from("test.svg");

    std::fs::write(&output, diagram.to_string().into_bytes()).unwrap();
}

fn make_node(pair: Pair<'_, Rule>) -> Box<dyn rr::Node> {
    use Rule as R;

    match pair.as_rule() {
        R::rule => {
            let mut pair = pair.into_inner();
            let rule = pair.next().expect("no rule found");
            let name = Box::new(rr::Comment::new(rule.as_str().to_owned())) as Box<dyn rr::Node>;

            let expr = pair.next().expect("rule must have definition").into_inner();
            let mut rule_def = expr.map(parse_expr).collect::<Vec<_>>();

            if rule_def.len() == 1 {
                let mut node = rule_def.remove(0);
                node.insert(0, name);
                Box::new(rr::Sequence::new(node))
            } else {
                let seqs = rule_def
                    .into_iter()
                    .map(|s| Box::new(rr::Sequence::new(s)) as Box<dyn rr::Node>)
                    .collect::<Vec<_>>();
                let x = vec![name, Box::new(rr::Choice::new(seqs))];

                Box::new(rr::Sequence::new(x))
            }
        }
        _ => {
            unreachable!("unhandled rule '{:?}' ({})", pair.as_rule(), pair.as_str());
        }
    }
}

fn parse_expr(pair: Pair<'_, Rule>) -> Vec<Box<dyn rr::Node>> {
    use Rule as R;

    let node = match pair.as_rule() {
        R::list => pair.into_inner().map(parse_term).collect::<Vec<_>>(),
        _ => {
            unreachable!("parse_expr unsupported rule: {:?}", pair.as_rule());
        }
    };

    node
}

fn parse_term(pair: Pair<'_, Rule>) -> Box<dyn rr::Node> {
    use Rule as R;

    match pair.as_rule() {
        R::term => {
            let rule_or_lit = pair.into_inner().next().expect("term must have inner");
            match rule_or_lit.as_rule() {
                R::rule_name => Box::new(rr::NonTerminal::new(rule_or_lit.as_str().to_owned())),
                R::literal => Box::new(rr::Terminal::new(rule_or_lit.as_str().to_owned())),
                _ => {
                    unreachable!("Got unsupported rule '{}' in term", rule_or_lit.as_str());
                }
            }
        }
        _ => {
            unreachable!("parse_expr unsupported rule: {:?}", pair.as_rule());
        }
    }
}
