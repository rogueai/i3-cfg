# i3-cfg

[![CI](https://github.com/rogueai/i3-cfg/actions/workflows/ci.yml/badge.svg)](https://github.com/rogueai/i3-cfg/actions/workflows/ci.yml)

This project aims at providing [i3wm](https://i3wm.org/) configuration in structured way, so that it could be used by 
other programs.

## Requirements

- `i3 version 4.14` or later

## Install and run
The project is built using Rust edition 2021, and requires Rust `v1.56.0` or later to build.

Build and run using Cargo:
```shell
cargo run --release
```

## Examples
### Sample output
```json
{
  "keybindings": [
    {
      "type": "keysym",
      "modifiers": [
        "Mod4"
      ],
      "key": "u",
      "criteria": "[class=\"Firefox\" window_role=\"About\"]",
      "command": "border none"
    },
    {
      "type": "keysym",
      "modifiers": [
        "Mod4"
      ],
      "key": "Return",
      "command": "exec kitty"
    },
    {
      "type": "keysym",
      "modifiers": [
        "Mod4",
        "Shift"
      ],
      "key": "q",
      "command": "kill"
    },
    {
      "type": "keysym",
      "modifiers": [
        "Mod4"
      ],
      "key": "t",
      "command": "exec --no-startup-id pkill picom"
    }
  ]
}
```
### Consume output using `jq`
```
❯ i3-cfg | jq '.keybindings[] | [([.modifiers, .key] | flatten | join("+")), .command] | join(" ")'
"Mod4+u border none"
"Mod4+Return exec kitty"
"Mod4+Shift+q kill"
"XF86MonBrightnessUp exec \"brightnessctl set +10%; notify-send 'brightness up'\""/
"Mod4+t exec --no-startup-id pkill picom"
```
## Implementation notes
i3 config is retrieved via IPC using [my fork](https://github.com/rogueai/i3ipc-rs) of [tmerr/i3ipc-rs](https://github.com/tmerr/i3ipc-rs), 
where I added a way to retrieve the `variable_replaced_config`.

The variable replaced config is then fed to a [Pest](https://pest.rs/) parser to construct the resulting data structure
which is finally serialized to Json.

## Current limitations
The goal is to eventually implement the full set of config directives, but so far only keybindings are implemented.

Even in the context of keybindings, the following features are not currently supported and will be added in future releases:

- multiline
- mode bindings
- configuration includes

> Disclaimer: this project is very much a work in progress, and it started as a way for me to learn Rust. It is probably
> not the most idiomatic Rust you can find, but hopefully it'll get better as I learn more about the language.
