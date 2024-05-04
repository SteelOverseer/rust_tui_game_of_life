use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{
    layout::{Constraint, Direction, Layout}, prelude::{CrosstermBackend, Stylize, Terminal}, widgets::{Block, Borders, Paragraph}
};
use std::{io::{stdout, Result}, thread, time};

#[derive(Debug, Clone, Copy)]
struct Cell {
    alive: bool
}

fn main() -> Result<()> {
    let max_rows = 5;
    let max_columns = 5;
    let mut game_of_life_grid = vec![vec![Cell {alive: false}; max_columns]; max_rows];

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
            let vertical_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                ])
                .split(frame.size());

            for i in 0..vertical_layout.len() {
                let horizontal_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![
                        Constraint::Percentage(20),
                        Constraint::Percentage(20),
                        Constraint::Percentage(20),
                        Constraint::Percentage(20),
                        Constraint::Percentage(20),
                    ])
                    .split(vertical_layout[i]);

                for j in 0..horizontal_layout.len() {
                    if game_of_life_grid[j][i].alive {
                        frame.render_widget(
                            Paragraph::new("")
                                .block(Block::new().borders(Borders::ALL))
                                .on_white()
                                .white(),
                                horizontal_layout[j]);
                    }
                }
            }
        })?;

        thread::sleep(time::Duration::from_millis(1000));

        game_of_life_grid = create_next_generation(&game_of_life_grid);

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press
                    && key.code == KeyCode::Char('q')
                {
                    break;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn create_next_generation(curr_gen: &Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    let n = curr_gen.len();
    let m = curr_gen[0].len();

    let mut future_gen = vec![vec![Cell {alive: false}; n]; m];

    for i in 0..n {
        for j in 0..m {
            let curr_cell = curr_gen[i][j];
            let mut live_neighbors = 0;

            // Check neighbors
            for x in -1i8..=1 {
                for y in -1i8..=1 {
                    let new_x = i as i8 + x;
                    let new_y = j as i8 + y;

                    if new_x > 0 && new_y > 0 && new_x < n as i8 && new_y < m as i8 {
                        let neighbor = curr_gen[new_x as usize][new_y as usize];
                        if neighbor.alive{
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
                future_gen[i][j].alive = false;
            } else if curr_cell.alive && live_neighbors > 3 {
                // Overpopulation - Dead
                future_gen[i][j].alive = false;
            } else if !curr_cell.alive && live_neighbors == 3 {
                // Reproduction - Live
                future_gen[i][j].alive = true;
            } else {
                // No change
                future_gen[i][j].alive = curr_cell.alive;
            }
        }
    }

    return future_gen;
}