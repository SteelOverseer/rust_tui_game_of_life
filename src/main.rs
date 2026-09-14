use crossterm::{
  event::{self, KeyCode, KeyEventKind},
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
  ExecutableCommand,
};
use ratatui::{
  layout::{Constraint, Direction, Flex, Layout, Spacing}, prelude::{CrosstermBackend, Stylize, Terminal}, symbols::merge::MergeStrategy, text::Line, widgets::{Block, BorderType, Borders, Paragraph},
};
use std::{
  io::{stdout, Result},
  time,
};

#[derive(Debug, Clone, Copy)]
struct Cell {
  alive: bool,
}

fn main() -> Result<()> {
  const CHAR_ASPECT_RATIO: u16 = 2;
  const SIMULATION_TICK: time::Duration = time::Duration::from_millis(1000);
  const INPUT_POLL_INTERVAL: time::Duration = time::Duration::from_millis(16);

  let max_rows = 5;
  let max_columns = 5;
  let mut game_of_life_grid = vec![vec![Cell { alive: false }; max_columns]; max_rows];
  let mut paused = false;
  let mut last_tick = time::Instant::now();

  // Init grid with a blinker
  game_of_life_grid[1][2].alive = true;
  game_of_life_grid[2][2].alive = true;
  game_of_life_grid[3][2].alive = true;

  stdout().execute(EnterAlternateScreen)?;
  enable_raw_mode()?;
  let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
  terminal.clear()?;

  loop {
    terminal.draw(|frame| {
      let [top, bottom] = Layout::vertical([Constraint::Percentage(75), Constraint::Fill(1)])
        .spacing(Spacing::Overlap(1))
        .areas(frame.area());

      let game_block = Block::bordered().title("Game of Life").merge_borders(MergeStrategy::Exact);
      let game_area = game_block.inner(top);
      let options_block = Block::bordered().title("Options").merge_borders(MergeStrategy::Exact);
      let options_area = options_block.inner(bottom);

      let max_cell_height_by_height = game_area.height / max_rows as u16;
      let max_cell_height_by_width = (game_area.width / max_columns as u16) / CHAR_ASPECT_RATIO;
      let cell_height = max_cell_height_by_height.min(max_cell_height_by_width).max(1);
      let cell_width = cell_height * CHAR_ASPECT_RATIO;

      frame.render_widget(game_block, top);
      frame.render_widget(options_block, bottom);

      // Pause Button
      let button_label = if paused { " [Resume] " } else { " [Pause] " };

      let [button_area] = Layout::horizontal([Constraint::Length(button_label.len() as u16 + 2)])
        .flex(Flex::Center)
        .areas(options_area);

      let [button_area] = Layout::vertical([Constraint::Length(3)])
        .flex(Flex::Center)
        .areas(button_area);

      frame.render_widget(
        Paragraph::new(button_label)
          .block(
            Block::bordered()
              .title_bottom(
                Line::from("(Space)")
                .right_aligned()
              )
              .border_type(BorderType::Double)
          )
          .on_black()
          .white()
          .bold()
          .centered(),
        button_area);
      // End Pause Button

      let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(cell_height); max_rows])
        .flex(Flex::Center)
        .split(game_area);

      for row in 0..vertical_layout.len() {
        let horizontal_layout = Layout::default()
          .direction(Direction::Horizontal)
          .constraints(vec![Constraint::Length(cell_width); max_columns])
          .flex(Flex::Center)
          .split(vertical_layout[row]);

        for col in 0..horizontal_layout.len() {
          if game_of_life_grid[row][col].alive {
            frame.render_widget(
              Paragraph::new("")
                .block(Block::new().borders(Borders::ALL))
                .on_white()
                .white(),
              horizontal_layout[col],
            );
          }
        }
      }
    })?;

    if event::poll(INPUT_POLL_INTERVAL)? {
      if let event::Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press {
          match key.code {
            KeyCode::Char('q') => break,
            KeyCode::Char(' ') => paused = !paused,
            _ => {}
          }
        }
      }
    }

    if !paused && last_tick.elapsed() >= SIMULATION_TICK {
      game_of_life_grid = create_next_generation(&game_of_life_grid);
      last_tick = time::Instant::now();
    }
  }

  stdout().execute(LeaveAlternateScreen)?;
  disable_raw_mode()?;
  Ok(())
}

fn create_next_generation(curr_gen: &Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
  let rows = curr_gen.len();
  let cols = curr_gen[0].len();

  let mut future_gen = vec![vec![Cell { alive: false }; cols]; rows];

  for row in 0..rows {
    for col in 0..cols {
      let curr_cell = curr_gen[row][col];
      let mut live_neighbors = 0;

      // Check neighbors
      for row_offset in -1i8..=1 {
        for col_offset in -1i8..=1 {
          let neighbor_row = row as i8 + row_offset;
          let neighbor_col = col as i8 + col_offset;

          if neighbor_row > 0 && neighbor_col > 0 && neighbor_row < rows as i8 && neighbor_col < cols as i8 {
            let neighbor = curr_gen[neighbor_row as usize][neighbor_col as usize];
            if neighbor.alive {
              live_neighbors += 1;
            }
          }
        }
      }

      if curr_cell.alive {
        live_neighbors -= 1;
      }

      if curr_cell.alive && live_neighbors < 2 {
        // Underpopulation - Dead
        future_gen[row][col].alive = false;
      } else if curr_cell.alive && live_neighbors > 3 {
        // Overpopulation - Dead
        future_gen[row][col].alive = false;
      } else if !curr_cell.alive && live_neighbors == 3 {
        // Reproduction - Live
        future_gen[row][col].alive = true;
      } else {
        // No change
        future_gen[row][col].alive = curr_cell.alive;
      }
    }
  }

  return future_gen;
}
