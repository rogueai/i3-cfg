use crate::model::{Config, Keybinding, KeybindingVariant};
use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar/i3-cfg.pest"]
pub struct I3Parser;

pub fn parse(config: String) {
    let config = I3Parser::parse(Rule::config, &config)
        .expect("parse failed") // unwrap the parse result
        .next()
        .unwrap(); // get and unwrap the `file` rule; never fails

    let mut keybindings: Vec<Keybinding> = vec![];
    parse_pair(config, &mut keybindings);

    let result = Config { keybindings };
    let json = serde_json::to_string_pretty(&result).unwrap();
    println!("{}", json);
}

fn parse_pair(parent: Pair<Rule>, keybindings: &mut Vec<Keybinding>) {
    for child in parent.into_inner() {
        match child.as_rule() {
            Rule::stmt => parse_pair(child, keybindings),
            Rule::expr => parse_pair(child, keybindings),
            Rule::keybinding => parse_pair(child, keybindings),
            Rule::binding => {
                keybindings.push(Keybinding::default());
                parse_pair(child, keybindings);
            }
            Rule::bindsym => {
                parse_pair(child, keybindings);
            }
            Rule::bindcode => {
                parse_pair(child, keybindings);
            }
            Rule::keycode => {
                let mut kb = keybindings.pop().unwrap();
                kb.variant = KeybindingVariant::Keycode {
                    code: child.as_str().parse().unwrap(),
                };
                keybindings.push(kb);
            }
            Rule::keysym => {
                let mut kb = keybindings.pop().unwrap();
                kb.variant = KeybindingVariant::Keysym {
                    key: child.as_str().parse().unwrap(),
                };
                keybindings.push(kb);
            }
            Rule::group => {
                let mut kb = keybindings.pop().unwrap();
                kb.group = Some(String::from(child.as_str()));
                keybindings.push(kb);
            }
            Rule::modifier => {
                let mut kb = keybindings.pop().unwrap();
                kb.modifiers.push(String::from(child.as_str()));
                keybindings.push(kb);
            }
            Rule::command => {
                let mut kb = keybindings.pop().unwrap();
                kb.command = String::from(child.as_str());
                keybindings.push(kb);
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
}
