use crossterm::{
  event::{self, KeyCode, KeyEventKind},
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
  ExecutableCommand,
};
use ratatui::{
  Frame, layout::{Constraint, Direction, Flex, Layout, Rect, Spacing}, prelude::{CrosstermBackend, Stylize, Terminal}, style::{Color, Style}, symbols::merge::MergeStrategy, widgets::{Block, BorderType, Borders, Paragraph},
};
use std::{
  io::{Result, stdout}, time::{self, Instant},
};

#[derive(Debug, Clone, Copy)]
struct Cell {
  alive: bool,
}

struct Settings {
  rows: usize,
  columns: usize,
  number_of_generations: Option<usize>,
  locked: bool,
  selected_pattern_index: usize,
  focused_field: Field,
  rows_buffer: String,
  columns_buffer: String,
  generations_buffer: String
}

impl Default for Settings {
  fn default() -> Self {
    Settings {
      rows: 5,
      columns: 5,
      number_of_generations: None,
      locked: false,
      selected_pattern_index: 0,
      focused_field: Field::Rows,
      rows_buffer: "5".to_string(),
      columns_buffer: "5".to_string(),
      generations_buffer: String::new()
    }
  }
}

impl Settings {
  fn is_valid(&self, field: Field) -> bool {
    match field {
      Field::Rows => is_positive(&self.rows_buffer),
      Field::Columns => is_positive(&self.columns_buffer),
      Field::Generations | Field::Pattern => true,
    }
  }

  fn first_invalid_field(&self) -> Option<Field> {
    Field::ALL.iter().copied().find(|field| !self.is_valid(*field))
  }
}

#[derive(Clone, Copy)]
enum Pattern {
  Blinker,
  Glider,
  Toad,
  Beacon,
  Pulsar,
  LightweightSpaceship
}

#[derive(Clone, Copy, PartialEq)]
enum Field {
  Rows,
  Columns,
  Generations,
  Pattern,
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

impl Field {
  const ALL: [Field; 4] = [Field::Rows, Field::Columns, Field::Generations, Field::Pattern];

  fn next(self) -> Field {
    let i = Field::ALL.iter().position(|f| *f == self).unwrap();
    Field::ALL[(i + 1) % Field::ALL.len()]
  }

  fn prev(self) -> Field {
    let i = Field::ALL.iter().position(|f| *f == self).unwrap();
    Field::ALL[(i + Field::ALL.len() - 1) % Field::ALL.len()]
  }
}

fn main() -> Result<()> {
  const CHAR_ASPECT_RATIO: u16 = 2;
  const SIMULATION_TICK: time::Duration = time::Duration::from_millis(1000);
  const INPUT_POLL_INTERVAL: time::Duration = time::Duration::from_millis(16);

  let mut paused = false;
  let mut last_tick = time::Instant::now();
  let mut settings = Settings::default();
  let mut game_of_life_grid = vec![vec![Cell { alive: false }; settings.columns]; settings.rows];
  let mut current_generation: u128 = 0;

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
        .title(format!(" Game of Life — Gen {current_generation} "))
        .merge_borders(MergeStrategy::Exact)
        .border_type(BorderType::Double);
      let game_area = game_block.inner(left);
      let options_block = Block::bordered()
        .title(" Options ")
        .merge_borders(MergeStrategy::Exact)
        .border_type(BorderType::Double);
      let options_area = options_block.inner(right);

      let max_cell_height_by_height = game_area.height / settings.rows as u16;
      let max_cell_height_by_width = (game_area.width / settings.columns as u16) / CHAR_ASPECT_RATIO;
      let cell_height = max_cell_height_by_height.min(max_cell_height_by_width).max(1);
      let cell_width = cell_height * CHAR_ASPECT_RATIO;

      frame.render_widget(game_block, left);
      frame.render_widget(options_block, right);

      let field_areas = Layout::vertical([Constraint::Length(3); Field::ALL.len() + 1])
        .flex(Flex::Start)
        .split(options_area);

      for (i, field) in Field::ALL.iter().enumerate() {
        render_settings_field(frame, field_areas[i], *field, &settings);
      } 

      render_legend(frame, *field_areas.last().unwrap(), paused, &settings);

      let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(cell_height); settings.rows])
        .flex(Flex::Center)
        .split(game_area);

      for row in 0..vertical_layout.len() {
        let horizontal_layout = Layout::default()
          .direction(Direction::Horizontal)
          .constraints(vec![Constraint::Length(cell_width); settings.columns])
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
            KeyCode::Enter => {
              if !settings.locked {
                if settings.first_invalid_field().is_none() {                  
                  settings.rows = settings.rows_buffer.parse().unwrap();
                  settings.columns = settings.columns_buffer.parse().unwrap();
                  settings.number_of_generations = settings.generations_buffer.parse().ok();

                  current_generation = 0;
                  game_of_life_grid = vec![vec![Cell { alive: false }; settings.columns]; settings.rows];

                  stamp_pattern(&mut game_of_life_grid, &settings);

                  settings.locked = true;
                }
              } 
              else {
                game_of_life_grid = vec![vec![Cell { alive: false }; settings.columns]; settings.rows];

                stamp_pattern(&mut game_of_life_grid, &settings);
                paused = false;
                last_tick = Instant::now();
                settings.locked = false;
              }
            }
            _ => {}
          }

          if !settings.locked {
            match key.code {
              KeyCode::Up => settings.focused_field = settings.focused_field.prev(),
              KeyCode::Down => settings.focused_field = settings.focused_field.next(),
              KeyCode::Left | KeyCode::Right => {
                let delta: i64 = if key.code == KeyCode::Left { -1 } else { 1 };
                match settings.focused_field {
                  Field::Pattern => {
                    let len = Pattern::ALL.len() as i64;
                    let idx = settings.selected_pattern_index as i64;
                    settings.selected_pattern_index = (idx + delta).rem_euclid(len) as usize;
                  }
                  Field::Rows => adjust_numeric_buffer(&mut settings.rows_buffer, delta, 1),
                  Field::Columns => adjust_numeric_buffer(&mut settings.columns_buffer, delta, 1),
                  Field::Generations => adjust_numeric_buffer(&mut settings.generations_buffer, delta, 0),
                }
              }
              _ => {}
            }
          }
        }
      }
    }

    if settings.locked && !paused && last_tick.elapsed() >= SIMULATION_TICK && settings.number_of_generations.is_none_or(|n| current_generation < n as u128) {
      game_of_life_grid = create_next_generation(&game_of_life_grid);
      current_generation += 1;
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

          if neighbor_row >= 0 && neighbor_col >= 0 && neighbor_row < rows as i8 && neighbor_col < cols as i8 {
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

fn render_legend(frame: &mut Frame, area: Rect, paused: bool, settings: &Settings) {
  let pause_button_label = if paused { "Resume [Space]" } else { "Pause [Space]" };
  let stop_button_label: &str = if settings.locked { "Stop [Enter]" } else { "Start [Enter]" };
  let quit_label: &str = "Quit [q]";

  let legend_lines = Layout::vertical([Constraint::Length(1); 3])
    .flex(Flex::Start)
    .split(area);

  for (label, line_area) in [pause_button_label, stop_button_label, quit_label].into_iter().zip(legend_lines.iter()) {
    frame.render_widget(
      Paragraph::new(label)
        .on_black()
        .white()
        .bold()
        .centered(),
      *line_area);
  }
}

fn render_settings_field(frame: &mut Frame, area: Rect, field: Field, settings: &Settings) {
  let label = field_display_text(field, settings);
  let is_focused = !settings.locked && field == settings.focused_field;

  let mut border_style = if !settings.is_valid(field) {
    Style::default().fg(Color::Red)
  } else if is_focused {
    Style::default().fg(Color::LightGreen)
  } else {
    Style::default()
  };

  let mut paragraph = Paragraph::new(label)
    .on_black()
    .white()
    .bold()
    .centered();

  if settings.locked {
    border_style = border_style.dim();
    paragraph = paragraph.dim();
  }

  frame.render_widget(
    paragraph.block(Block::bordered().border_style(border_style)),
    area);
}

fn stamp_pattern(grid: &mut Vec<Vec<Cell>>, settings: &Settings) {
  let pattern = Pattern::ALL[settings.selected_pattern_index];
  let cells = pattern.cells();
  let height = cells.iter().map(|(r, _)| r + 1).max().unwrap_or(0) as usize;
  let width = cells.iter().map(|(_, c)| c + 1).max().unwrap_or(0) as usize;

  let origin = (
    settings.rows.saturating_sub(height) / 2,
    settings.columns.saturating_sub(width) / 2,
  );

  for (row_offset, col_offset) in pattern.cells() {
    let row = origin.0 as isize + row_offset;
    let col = origin.1 as isize + col_offset;

    if row >= 0 && col >= 0 && (row as usize) < grid.len() && (col as usize) < grid[0].len() {
      grid[row as usize][col as usize].alive = true;
    }
  }
}

fn adjust_numeric_buffer(buffer: &mut String, delta: i64, min: i64) {
  let current: i64 = buffer.parse().unwrap_or(0);
  let updated = (current + delta).max(min);
  *buffer = updated.to_string();
}

fn is_positive(buffer: &str) -> bool {
  buffer.parse::<usize>().is_ok_and(|n| n > 0)
}

fn field_display_text(field: Field, settings: &Settings) -> String {
  match field {
    Field::Rows => format!("Rows: < {} >", settings.rows_buffer),
    Field::Columns => format!("Columns: < {} >", settings.columns_buffer),
    Field::Generations => {
      if settings.generations_buffer.is_empty() {
        "Generations: < \u{221E} >".to_string()  // ∞
      } else {
        format!("Generations: < {} >", settings.generations_buffer)
      }
    }
    Field::Pattern => {
      let pattern = Pattern::ALL[settings.selected_pattern_index];
      format!("< {} >", pattern.label())
    }
  }
}
