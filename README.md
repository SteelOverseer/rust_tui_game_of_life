# rust_tui_game_of_life

Conway's Game of Life, rendered as a terminal UI with [ratatui](https://ratatui.rs) and [crossterm](https://github.com/crossterm-rs/crossterm). A personal project for learning Rust and TUI development.

## Download

Prebuilt binaries for Linux, Windows, and macOS (Intel and Apple Silicon) are attached to the [latest release](https://github.com/SteelOverseer/rust_tui_game_of_life/releases/latest). On Linux and macOS, downloaded binaries aren't executable by default and have to be run from a terminal:

```
chmod +x <downloaded-binary>
./<downloaded-binary>
```

## Running it

```
cargo run
```

Renders to the alternate terminal screen. Use the sidebar to configure the grid size, an optional generation limit, and a starting pattern before locking in and starting the simulation. Controls:

| Key | Action |
| --- | --- |
| `Enter` | Lock in settings and start the simulation; while running, stop and return to the settings screen |
| `Up` / `Down` | Move focus between settings fields (while unlocked) |
| `Left` / `Right` | Adjust the focused field's value, or cycle the seed pattern (while unlocked) |
| `Space` | Pause / resume the simulation |
| `q` | Quit |

The game panel's title shows the current generation count. If a generation limit is set, the simulation stops advancing once it's reached.

## Building

```
cargo build     # build
cargo check     # type-check without producing a binary
cargo fmt       # format
cargo clippy    # lint
```

## Using Claude as a learning tool

I'm using [Claude Code](https://claude.com/claude-code) to help learn Rust and TUI development on this project. I write the code myself; Claude mostly explains concepts, reviews what I've written, and points out bugs or tradeoffs. I'll ask it to implement or fix something directly sometimes, but that's not the default.
