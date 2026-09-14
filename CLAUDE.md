# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

- Build: `cargo build`
- Run: `cargo run` (renders to the alternate terminal screen; press `q` to quit)
- Check without building a binary: `cargo check`
- Format: `cargo fmt`
- Lint: `cargo clippy`

There are no automated tests in this repository yet.

## Architecture

This is a terminal UI implementation of Conway's Game of Life, built on `crossterm` (raw terminal mode / input) and `ratatui` (rendering). Everything currently lives in `src/main.rs`.

- `Cell { alive: bool }` is the sole unit of grid state, held in `Vec<Vec<Cell>>` (`game_of_life_grid`), indexed `[row][col]`.
- `main()` drives a single loop that: renders the grid via `terminal.draw`, sleeps 1000ms, advances the simulation with `create_next_generation`, then polls for a `q` keypress to exit. Rendering and simulation are not decoupled from timing — the frame rate and simulation step are the same 1s tick.
- Rendering builds the grid by nesting `ratatui` layouts: an outer `Direction::Vertical` split into equal-percentage rows, then for each row an inner `Direction::Horizontal` split into equal-percentage columns. Each alive cell is drawn as a bordered, white-filled `Paragraph` in its layout cell. The percentage constraints are hardcoded per grid dimension, so resizing the grid (`max_rows`/`max_columns` in `main`) requires updating the constraint lists to match.
- `create_next_generation` computes the next state by checking the 8 neighbors of each cell (with wraparound-free bounds checking) and applying the standard Game of Life rules (underpopulation <2, survival on 2-3, overpopulation >3, reproduction on exactly 3).
- Note: the render loop indexes the grid as `game_of_life_grid[j][i]` (col, row) while `create_next_generation` indexes as `curr_gen[i][j]` (row, col) — be careful about this inconsistency when modifying grid access.
