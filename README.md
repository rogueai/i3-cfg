# i3-cfg

[![CI](https://github.com/rogueai/i3-cfg/actions/workflows/ci.yml/badge.svg)](https://github.com/rogueai/i3-cfg/actions/workflows/ci.yml)

This project aims a providing [i3wm](https://i3wm.org/) configuration in structured way, so that it could be used by 
other programs.

## Example output

```json
{
  "keybindings": [
    {
      "modifiers": [
        "Mod4"
      ],
      "key": "y",
      "command": "border pixel 1"
    },
    {
      "modifiers": [
        "Mod4",
        "Shift"
      ],
      "key": "q",
      "command": "kill"
    }
  ],
  <...>
}
```
## Requirements

- `i3 version 4.14` or later

## Install and run

Build and run using Cargo:
```shell
cargo build
cargo run
```

## Implementation notes
i3 config is retrieved via IPC using [my fork of tmerr/i3ipc-rs](https://github.com/rogueai/i3ipc-rs), where I added
a way to retrieve the `variable_replaced_config`.

The variable replaced config is then fed to a [Pest](https://pest.rs/) parser to construct the resulting data structure
which is finally serialized to Json.

## Current limitations
The goal is to eventually implement the full set of config directives, but so far only keybindings are implemented.

Even in the context of keybindings, the following features are not supported and will be added in future releases:

- multiline
- mode bindings
- configuration includes

> Disclaimer: this project is very much a work in progress, and it started as a way for me to learn Rust. It is probably
> not the most idiomatic Rust you can find, but hopefully it'll get better as I learn more about the language.
