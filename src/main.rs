#![allow(unused, clippy::all)]

use std::path::PathBuf;

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use railroad as rr;

type DynNode = Box<dyn rr::Node>;

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
                Box::new(rr::SimpleStart) as DynNode,
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
            let name = Box::new(rr::Comment::new(rule.as_str().to_owned())) as DynNode;

            let expr = pair.next().expect("rule must have definition").into_inner();
            let mut rule_def = expr.map(make_node).collect::<Vec<_>>();

            if rule_def.len() == 1 {
                let mut node = rule_def.remove(0);
                Box::new(rr::Sequence::new(vec![name, node]))
            } else {
                let x = vec![name, Box::new(rr::Choice::new(rule_def))];

                Box::new(rr::Sequence::new(x))
            }
        }
        R::list => {
            let seq = pair.into_inner().map(parse_term).collect::<Vec<_>>();
            Box::new(rr::Sequence::new(seq))
        }
        R::grouped_list => {
            let mut pairs = pair.into_inner();
            let pair = pairs.next().unwrap();
            let list = pair.into_inner();
            let nodes = list.map(make_node).collect::<Vec<_>>();

            let modifier = pairs.next().unwrap();
            let grouped_list = Box::new(rr::Sequence::new(nodes));
            parse_modifier(grouped_list, modifier)
        }
        _ => {
            unreachable!("unhandled rule '{:?}' ({})", pair.as_rule(), pair.as_str());
        }
    }
}

fn parse_term(pair: Pair<'_, Rule>) -> DynNode {
    use Rule as R;

    let mut pairs = pair.into_inner();
    let pair = pairs.next().unwrap();
    let grammar_rule = pair.as_rule();

    let node: DynNode = match grammar_rule {
        R::literal => Box::new(rr::Terminal::new(unescape(&pair))),
        R::rule_name => Box::new(rr::Terminal::new(pair.as_str().to_owned())),
        R::grouped_list => make_node(pair),
        _ => {
            unreachable!()
        }
    };
    let modifier = pairs.next().unwrap();
    parse_modifier(node, modifier)
}

fn new_optional(node: DynNode) -> DynNode {
    Box::new(rr::Optional::new(node))
}
fn new_repeat(node: DynNode, r: DynNode) -> DynNode {
    Box::new(rr::Repeat::new(node, r))
}

fn parse_modifier(node: DynNode, opt: Pair<'_, Rule>) -> DynNode {
    let modifier = opt.into_inner().next();

    if let Some(m) = modifier {
        use Rule as R;

        match m.as_rule() {
            R::oper_cond => Box::new(rr::Optional::new(node)),
            R::oper_alo => Box::new(rr::Repeat::new(node, rr::Empty)),
            R::oper_rep => Box::new(rr::Optional::new(rr::Repeat::new(node, rr::Empty))),
            _ => {
                dbg!(&m);
                unreachable!("\n\not invalid rule in parse_modifier\n\n")
            }
        }
    } else {
        node
    }
}

// Modified from https://github.com/lukaslueg/railroad_dsl/blob/06841c393b323c83925304011d965c43a10127e7/src/lib.rs#L19
fn unescape(pair: &Pair<'_, Rule>) -> String {
    let s = pair.as_str();
    let mut result = String::with_capacity(s.len());
    let mut iter = s[1..s.len() - 1].chars();
    while let Some(ch) = iter.next() {
        result.push(match ch {
            '\\' => {
                let mut peekable = iter.clone().peekable();
                let escaped = peekable.peek().expect("no escaped char?");
                if ['"', '\'', '\\'].contains(escaped) {
                    iter.next().unwrap()
                } else {
                    ch
                }
            }
            _ => ch,
        });
    }
    result
}
