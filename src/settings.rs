#[derive(Clone, Copy)]
pub enum Pattern {
  Blinker,
  Glider,
  Toad,
  Beacon,
  Pulsar,
  LightweightSpaceship
}

#[derive(Clone, Copy, PartialEq)]
pub enum Field {
  Rows,
  Columns,
  Generations,
  Pattern,
}

pub struct Settings {
  pub rows: usize,
  pub columns: usize,
  pub number_of_generations: Option<usize>,
  pub locked: bool,
  pub selected_pattern_index: usize,
  pub focused_field: Field,
  pub rows_buffer: String,
  pub columns_buffer: String,
  pub generations_buffer: String
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
  pub fn is_valid(&self, field: Field) -> bool {
    match field {
      Field::Rows => is_positive(&self.rows_buffer),
      Field::Columns => is_positive(&self.columns_buffer),
      Field::Generations | Field::Pattern => true,
    }
  }

  pub fn first_invalid_field(&self) -> Option<Field> {
    Field::ALL.iter().copied().find(|field| !self.is_valid(*field))
  }
}

pub fn adjust_numeric_buffer(buffer: &mut String, delta: i64, min: i64) {
  let current: i64 = buffer.parse().unwrap_or(0);
  let updated = (current + delta).max(min);
  *buffer = updated.to_string();
}

impl Pattern {
  pub const ALL: [Pattern; 6] = [
    Pattern::Blinker, Pattern::Glider, Pattern::Toad,
    Pattern::Beacon, Pattern::Pulsar, Pattern::LightweightSpaceship,
  ];

  pub fn cells(&self) -> &'static [(isize, isize)] {
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

  pub fn label(&self) -> &'static str {
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
  pub const ALL: [Field; 4] = [Field::Rows, Field::Columns, Field::Generations, Field::Pattern];

  pub fn next(self) -> Field {
    let i = Field::ALL.iter().position(|f| *f == self).unwrap();
    Field::ALL[(i + 1) % Field::ALL.len()]
  }

  pub fn prev(self) -> Field {
    let i = Field::ALL.iter().position(|f| *f == self).unwrap();
    Field::ALL[(i + Field::ALL.len() - 1) % Field::ALL.len()]
  }
}

fn is_positive(buffer: &str) -> bool {
  buffer.parse::<usize>().is_ok_and(|n| n > 0)
}