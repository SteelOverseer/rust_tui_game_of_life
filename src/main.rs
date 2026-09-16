mod game;
mod settings;
mod ui;

use crossterm::{
  event::{self, KeyCode, KeyEventKind},
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
  ExecutableCommand,
};
use ratatui::{
  layout::{Constraint, Direction, Flex, Layout, Spacing}, prelude::{CrosstermBackend, Stylize, Terminal}, symbols::merge::MergeStrategy, widgets::{Block, BorderType, Borders, Paragraph},
};
use std::{
  io::{Result, stdout}, time::{self, Instant},
};

use crate::{
  game::{Cell, create_next_generation, stamp_pattern},
  settings::{Field, Pattern, Settings, adjust_numeric_buffer},
  ui::{render_legend, render_settings_field},
};


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
