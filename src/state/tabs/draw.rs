use ratatui::{
  layout::{self},
  widgets::{Block, Borders, List},
};
use std::error::Error;

use ratatui::DefaultTerminal;

use crate::state::tabs::App;
#[allow(dead_code)]
impl App {
  pub fn draw_home_page(
    &self,
    terminal: &mut DefaultTerminal,
  ) -> std::result::Result<(), Box<dyn Error>> {
    let mut state = ratatui::widgets::ListState::default();
    state.select(Some(self.menu.selected));
    terminal.draw(|frame| {
      let list = List::new(self.menu.items.iter().copied())
        .block(Block::new().borders(Borders::ALL).title("Menu"))
        .highlight_style(ratatui::style::Style::default().bg(ratatui::style::Color::Cyan))
        .highlight_symbol("> ");
      frame.render_stateful_widget(list, frame.area(), &mut state);
    })?;
    Ok(())
  }

  pub fn draw_help_page(
    &self,
    terminal: &mut DefaultTerminal,
  ) -> std::result::Result<(), Box<dyn Error>> {
    terminal.draw(|frame| {
      let [help_page] =
        ratatui::layout::Layout::vertical([layout::Constraint::Fill(1)]).areas(frame.area());
      frame.render_widget(
        ratatui::widgets::Paragraph::new("This is the Help Page").block(
          ratatui::widgets::Block::new()
            .borders(ratatui::widgets::Borders::ALL)
            .title("Help Page"),
        ),
        help_page,
      );
    })?;
    Ok(())
  }
}
