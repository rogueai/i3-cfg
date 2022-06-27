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
            Rule::stmt
            | Rule::expr
            | Rule::keybinding
            | Rule::bindsym
            | Rule::bindcode
            | Rule::bindmouse => {
                parse_pair(child, keybindings);
            }
            Rule::binding_sym | Rule::binding_code | Rule::binding_mouse => {
                keybindings.push(Keybinding::default());
                parse_pair(child, keybindings);
            }
            Rule::keycode => {
                let mut kb = keybindings.pop().unwrap();
                kb.variant = KeybindingVariant::Keycode {
                    keycode: child.as_str().parse().unwrap(),
                };
                keybindings.push(kb);
            }
            Rule::keysym => {
                let mut kb = keybindings.pop().unwrap();
                kb.variant = KeybindingVariant::Keysym {
                    keysym: child.as_str().parse().unwrap(),
                };
                keybindings.push(kb);
            }
            Rule::button => {
                let mut kb = keybindings.pop().unwrap();
                kb.variant = KeybindingVariant::Button {
                    button: child.as_str().parse().unwrap(),
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