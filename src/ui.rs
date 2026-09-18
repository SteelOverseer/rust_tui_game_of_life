use ratatui::{Frame, layout::{Constraint, Flex, Layout, Rect}, style::{Color, Style, Stylize}, widgets::{Block, Paragraph}};

use crate::settings::{Field, Pattern, Settings};

pub fn render_legend(frame: &mut Frame, area: Rect, paused: bool, settings: &Settings) {
  let pause_button_label = if paused { "Resume [Space]" } else { "Pause [Space]" };
  let stop_button_label: &str = if settings.locked { "Stop [Enter]" } else { "Start [Enter]" };
  let quit_label: &str = "Quit [q]";

  let legend_lines = Layout::vertical([Constraint::Length(1); 3])
    .flex(Flex::Start)
    .split(area);

  for (label, line_area) in [stop_button_label, pause_button_label, quit_label].into_iter().zip(legend_lines.iter()) {
    frame.render_widget(
      Paragraph::new(label)
        .on_black()
        .white()
        .bold()
        .centered(),
      *line_area);
  }
}

pub fn render_settings_field(frame: &mut Frame, area: Rect, field: Field, settings: &Settings) {
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