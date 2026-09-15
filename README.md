# rust_tui_game_of_life

Conway's Game of Life, rendered as a terminal UI with [ratatui](https://ratatui.rs) and [crossterm](https://github.com/crossterm-rs/crossterm). A personal project for learning Rust and TUI development.

## Running it

```
cargo run
```

Renders to the alternate terminal screen. Controls:

| Key | Action |
| --- | --- |
| `q` | Quit |
| `Space` | Pause / resume the simulation |

A settings sidebar (grid size, generation limit, seed patterns) is under active development.

## Building

```
cargo build     # build
cargo check     # type-check without producing a binary
cargo fmt       # format
cargo clippy    # lint
```

## Using Claude as a learning tool

I'm using [Claude Code](https://claude.com/claude-code) to help learn Rust and TUI development on this project. I write the code myself; Claude mostly explains concepts, reviews what I've written, and points out bugs or tradeoffs. I'll ask it to implement or fix something directly sometimes, but that's not the default.
