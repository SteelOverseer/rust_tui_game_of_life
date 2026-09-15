use crossterm::{
  event::{self, KeyCode, KeyEventKind},
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
  ExecutableCommand,
};
use ratatui::{
  Frame, layout::{Constraint, Direction, Flex, Layout, Rect, Spacing}, prelude::{CrosstermBackend, Stylize, Terminal}, symbols::merge::MergeStrategy, text::Line, widgets::{Block, BorderType, Borders, Paragraph},
};
use std::{
  io::{stdout, Result},
  time,
};

#[derive(Debug, Clone, Copy)]
struct Cell {
  alive: bool,
}

struct Settings {
  rows: usize,
  columns: usize,
  number_of_generations: Option<usize>,
  locked: bool
}

impl Default for Settings {
  fn default() -> Self {
    Settings { 
      rows: 5,
      columns: 5,
      number_of_generations: None,
      locked: false
    }
  }
}

enum Pattern {
  Blinker,
  Glider,
  Toad,
  Beacon,
  Pulsar,
  LightweightSpaceship
}

impl Pattern {
  const ALL: [Pattern; 6] = [
    Pattern::Blinker, Pattern::Glider, Pattern::Toad,
    Pattern::Beacon, Pattern::Pulsar, Pattern::LightweightSpaceship,
  ];

  fn cells(&self) -> &'static [(isize, isize)] {
    match self {
      Pattern::Blinker => &[(0, 0), (0, 1), (0, 2)],
      Pattern::Glider => &[(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)],
      Pattern::Toad => &[
        (0, 1), (0, 2), (0, 3),
        (1, 0), (1, 1), (1, 2)
      ],
      Pattern::Beacon => &[
        (0, 0), (0, 1),
        (1, 0), (1, 1),
        (2, 2), (2, 3),
        (3, 2), (3, 3)
      ],
      Pattern::Pulsar => &[
        (0, 2), (0, 3), (0, 4), (0, 8), (0, 9), (0, 10),
        (5, 2), (5, 3), (5, 4), (5, 8), (5, 9), (5, 10),
        (7, 2), (7, 3), (7, 4), (7, 8), (7, 9), (7, 10),
        (12, 2), (12, 3), (12, 4), (12, 8), (12, 9), (12, 10),
        (2, 0), (2, 5), (2, 7), (2, 12),
        (3, 0), (3, 5), (3, 7), (3, 12),
        (4, 0), (4, 5), (4, 7), (4, 12),
        (8, 0), (8, 5), (8, 7), (8, 12),
        (9, 0), (9, 5), (9, 7), (9, 12),
        (10, 0), (10, 5), (10, 7), (10, 12)
      ],
      Pattern::LightweightSpaceship => &[
        (0, 1), (0, 2), (0, 3), (0, 4),
        (1, 0), (1, 4),
        (2, 4),
        (3, 0), (3, 3)
      ]
    }
  }

  fn label(&self) -> &'static str {
    match self {
      Pattern::Blinker => "Blinker",
      Pattern::Glider => "Glider",
      Pattern::Toad => "Toad",
      Pattern::Beacon => "Beacon",
      Pattern::Pulsar => "Pulsar",
      Pattern::LightweightSpaceship => "Lightweight Spaceship"
    }
  }
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
      let [left, right] = Layout::horizontal([Constraint::Percentage(75), Constraint::Fill(1)])
        .spacing(Spacing::Overlap(1))
        .areas(frame.area());

      let game_block = Block::bordered()
        .title("Game of Life")
        .merge_borders(MergeStrategy::Exact)
        .border_type(BorderType::Double);
      let game_area = game_block.inner(left);
      let options_block = Block::bordered()
        .title("Options")
        .merge_borders(MergeStrategy::Exact)
        .border_type(BorderType::Double);
      let options_area = options_block.inner(right);

      let max_cell_height_by_height = game_area.height / max_rows as u16;
      let max_cell_height_by_width = (game_area.width / max_columns as u16) / CHAR_ASPECT_RATIO;
      let cell_height = max_cell_height_by_height.min(max_cell_height_by_width).max(1);
      let cell_width = cell_height * CHAR_ASPECT_RATIO;

      frame.render_widget(game_block, left);
      frame.render_widget(options_block, right);

      render_pause_button(frame, options_area, paused);

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

fn render_pause_button(frame: &mut Frame, area: Rect, paused: bool) {
  let button_label = if paused { " [Resume] " } else { " [Pause] " };

  let [button_area] = Layout::horizontal([Constraint::Length(button_label.len() as u16 + 2)])
    .flex(Flex::Center)
    .areas(area);

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
          .border_type(BorderType::Plain)
      )
      .on_black()
      .white()
      .bold()
      .centered(),
    button_area);
}

fn stamp_pattern(grid: &mut Vec<Vec<Cell>>, pattern: Pattern, origin: (usize, usize)) {
  for (row_offset, col_offset) in pattern.cells() {
    let row = origin.0 as isize + row_offset;
    let col = origin.1 as isize + col_offset;

    if row >= 0 && col >= 0 && (row as usize) < grid.len() && (col as usize) < grid[0].len() {
      grid[row as usize][col as usize].alive = true;
    }
  }
}
