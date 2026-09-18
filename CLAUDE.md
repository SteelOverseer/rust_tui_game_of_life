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

This is a terminal UI implementation of Conway's Game of Life, built on `crossterm` (raw terminal mode / input) and `ratatui` (rendering). Code is split across four files:

- `src/game.rs` — pure simulation logic, no `ratatui`/`crossterm` dependency. `Cell { alive: bool }` is the sole unit of grid state, held in `Vec<Vec<Cell>>` and indexed consistently as `[row][col]` everywhere. `stamp_pattern` centers a `Pattern`'s cells within the current `settings.rows`/`settings.columns` (bounds-checked, so oversized patterns are silently clipped). `create_next_generation` computes the next generation via the standard rules (underpopulation <2, survival on 2-3, overpopulation >3, reproduction on exactly 3).
- `src/settings.rs` — all settings/input state. `Settings` holds the live `rows`/`columns`/`number_of_generations` plus per-field `String` buffers (`rows_buffer`, `columns_buffer`, `generations_buffer`) that back in-place editing before a value is parsed; `locked` gates whether the sidebar is still editable. `Field` (`Rows`/`Columns`/`Generations`/`Pattern`) and `Pattern` (6 variants, each with a fixed cell layout) are cycled via `ALL` arrays + `next()`/`prev()`/`rem_euclid`-based index wraparound rather than being enum-native. `Settings::is_valid`/`first_invalid_field` gate whether `Enter` is allowed to lock in settings (Rows/Columns must parse to a positive `usize`); `adjust_numeric_buffer` clamps a field's buffer at a caller-supplied minimum (1 for Rows/Columns so they can never reach an invalid `0`, 0 for Generations).
- `src/ui.rs` — rendering only. `render_settings_field` draws one bordered field box (red border if invalid, green if focused, dimmed once locked); `render_legend` draws the three-line Pause/Start-Stop/Quit control legend; `field_display_text` formats each field's label (`"Generations: < ∞ >"` when the buffer is empty, meaning no limit).
- `src/main.rs` — wires it together. `main()`'s loop: `terminal.draw` renders the game/options split layout (cell size computed from `settings.rows`/`columns` each frame, so the grid is fully dynamic — no hardcoded dimensions), then handles input, then advances the simulation on a 1s tick gated on `settings.locked && !paused` and an optional generation limit (`settings.number_of_generations.is_none_or(...)`, since `None` means unlimited).
- `Enter` is the core interaction, handled in `main()`'s input match: while unlocked, it validates + parses the settings buffers, resizes/stamps a fresh grid, resets `current_generation`, and locks; while locked, it resets/re-stamps the grid and unlocks back to the settings screen, resetting `paused`/`last_tick` but *not* `current_generation` (the title's counter carries over from the previous run rather than resetting to 0 — worth a look if that's ever surprising in practice). Arrow keys (`Up`/`Down` to change focus, `Left`/`Right` to adjust the focused field or cycle the pattern) only take effect while unlocked.
- The game panel's title shows the live generation count (`" Game of Life — Gen {current_generation} "`), recomputed each frame.
- A GitHub Actions workflow (`.github/workflows/release.yml`) builds Linux/Windows/macOS (Intel + Apple Silicon) release binaries on `git tag v*` push and attaches them to a GitHub Release.
