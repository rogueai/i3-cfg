use crate::model::{Config, Keybinding, KeybindingType};
use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar/i3-cfg.pest"]
pub struct I3Parser;

pub fn parse(config: String) -> Config {
    let config = I3Parser::parse(Rule::config, &config)
        .expect("parse failed") // unwrap the parse result
        .next()
        .unwrap(); // get and unwrap the `file` rule; never fails

    let mut keybindings: Vec<Keybinding> = vec![];
    parse_pair(config, &mut keybindings);

    Config { keybindings }
}
/// Recursively parse [Pair]s and populate keybindings.
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
                kb.variant = KeybindingType::Keycode;
                kb.key = child.as_str().parse().unwrap();
                keybindings.push(kb);
            }
            Rule::keysym => {
                let mut kb = keybindings.pop().unwrap();
                kb.variant = KeybindingType::Keysym;
                kb.key = child.as_str().parse().unwrap();
                keybindings.push(kb);
            }
            Rule::button => {
                let mut kb = keybindings.pop().unwrap();
                kb.variant = KeybindingType::Button;
                kb.key = child.as_str().parse().unwrap();
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
            Rule::criteria => {
                let mut kb = keybindings.pop().unwrap();
                kb.criteria = Some(String::from(child.as_str()));
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

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_json_snapshot;

    #[test]
    fn test_comment() {
        let config = r###"
        # this is a comment
        # and another one
        "###;
        let config = parse(String::from(config));
        assert_json_snapshot!(config);
    }

    #[test]
    fn test_mode() {
        let config = r###"
        mode "resize" {
            # Pressing down will grow the window’s height.
            bindsym j resize shrink width 5 px or 5 ppt
            bindsym k resize grow height 5 px or 5 ppt
            bindsym l resize shrink height 5 px or 5 ppt
            bindsym semicolon resize grow width 5 px or 5 ppt
        }
        "###;
        let config = parse(String::from(config));
        assert_json_snapshot!(config);
    }

    #[test]
    fn test_bar() {
        let config = r###"
        bar {
            # Pressing down will grow the window’s height.
            bindsym j resize shrink width 5 px or 5 ppt
            bindsym k resize grow height 5 px or 5 ppt
            bindsym l resize shrink height 5 px or 5 ppt
            bindsym semicolon resize grow width 5 px or 5 ppt
        }
        "###;
        let config = parse(String::from(config));
        assert_json_snapshot!(config);
    }

    #[test]
    fn test_bindsym_modifier_1() {
        let config = r###"
        bindsym Mod4+u border none
        "###;
        let config = parse(String::from(config));
        assert_json_snapshot!(config);
    }

    #[test]
    fn test_bindsym_modifier_2() {
        let config = r###"
        bindsym Mod4+Shift+r restart
        "###;
        let config = parse(String::from(config));
        assert_json_snapshot!(config);
    }

    #[test]
    fn test_bindsym_release() {
        let config = r###"
        bindsym --release Mod4+x exec --no-startup-id import /tmp/latest-screenshot.png
        "###;
        let config = parse(String::from(config));
        assert_json_snapshot!(config);
    }

    #[test]
    fn test_bindcode_1() {
        let config = r###"
        bindcode 214 exec --no-startup-id /home/michael/toggle_beamer.sh
        "###;
        let config = parse(String::from(config));
        assert_json_snapshot!(config);
    }

    #[test]
    fn test_bindmouse_1() {
        let config = r###"
        bindsym --release button2 kill
        "###;
        let config = parse(String::from(config));
        assert_json_snapshot!(config);
    }

    #[test]
    fn test_bindmouse_2() {
        let config = r###"
        bindsym --whole-window Mod4+button2 kill
        "###;
        let config = parse(String::from(config));
        assert_json_snapshot!(config);
    }

    #[test]
    fn test_bindmouse_3() {
        let config = r###"
        bindsym button8 move right
        "###;
        let config = parse(String::from(config));
        assert_json_snapshot!(config);
    }
}
