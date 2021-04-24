# Quest-tui
a simple todo list terminal user interface
![Demo](/images/demo.png)

### Built With
- [Rust🦀](https://www.rust-lang.org/)
- [tui-rs](https://github.com/fdehau/tui-rs)

## Getting Started
Steps to install and run the application in your terminal.

### Prerequisites
- Rust ([installtion guide](https://www.rust-lang.org/tools/install))

### Installation
- `cargo install quest-tui`

### Building Locally
Clone the project then:
- `cargo build` to build iit
- `cargo run` to run it directly

## Usage
after installing with cargo, you can run the command `quest-tui`

## Configuration
Quest uses [directories-next](https://crates.io/crates/directories-next) to save configuration and data files,
config directory:
- Linux:   `$XDG_CONFIG_HOME/quest or $HOME/.config/quest`
- Windows: `{FOLDERID_RoamingAppData}/quest/config`
- macOS:   `$HOME/Library/Application Support/quest`
A config file is created if one does not exists already when you run the app.

default configs:
```json
{
    "colors": {
      "foreground": "White",
      "background": "Black",
      "selection_fg": "Black",
      "selection_bg": "Yellow",
      "check_sign": "Green"
    },
    "keybindings": {
      "exit_app": {
        "Char": "q"
      },
      "new_quest": {
        "Char": "n"
      },
      "check_and_uncheck_quest": "Enter",
      "list_up": "Up",
      "list_down": "Down",
      "delete_quest": "Delete",
      "exit_adding": "Esc",
      "save_quest": "Enter"
    }
}
```
You can refer to [Color](https://docs.rs/tui/0.6.0/tui/style/enum.Color.html) and [Keybinding](https://docs.rs/crossterm/0.17.7/crossterm/event/enum.KeyCode.html) for all possible configs.

### Example keybindings

#### Vi
```json
  "keybindings": {
    "exit_app": {
      "Char": "z"
    },
    "new_quest": {
      "Char": "n"
    },
    "check_and_uncheck_quest": {
      "Char": "c"
    },
    "list_up": {
      "Char": "k"
    },
    "list_down": {
      "Char": "j"
    },
    "delete_quest": {
      "Char": "x"
    },
    "exit_adding": "Esc",
    "save_quest": "Enter"
```

## Contributing
Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are greatly appreciated.
[guide](https://github.com/frustak/Quest/blob/main/CONTRIBUTING.md)

## Authors
- [frustak](https://github.com/frustak) - *Initial work*

## Acknowledgments
- [Simon Scatton](https://github.com/SDAChess) - *Help with Github Porject*

## License
Distributed under the MIT License. See [LICENSE](https://github.com/frustak/Quest/blob/main/LICENSE) for more information.
