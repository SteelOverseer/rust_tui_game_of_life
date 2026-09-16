use crate::settings::{Pattern, Settings};

#[derive(Debug, Clone, Copy)]
pub struct Cell {
  pub alive: bool,
}

pub fn stamp_pattern(grid: &mut Vec<Vec<Cell>>, settings: &Settings) {
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

pub fn create_next_generation(curr_gen: &Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
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