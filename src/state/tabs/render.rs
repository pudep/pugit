use crate::state::tabs::App;
use ratatui::{
  Frame,
  layout::{self, Constraint},
  widgets::{Block, Borders, Paragraph},
};

#[allow(dead_code)]
impl App {
  pub fn render_homepage(frame: &mut Frame) {
    let [homepage] = ratatui::layout::Layout::vertical([Constraint::Fill(1)]).areas(frame.area());

    frame.render_widget(
      Paragraph::new("This is the Homepage").block(Block::new().borders(Borders::ALL).title("Home Page")),
      homepage,
    );
  }
  pub fn render_help_page(frame: &mut Frame) {
    let [help_page] =
      ratatui::layout::Layout::vertical([layout::Constraint::Fill(1)]).areas(frame.area());
    frame.render_widget(
      ratatui::widgets::Paragraph::new("This is the Help Page")
        .block(ratatui::widgets::Block::new().borders(ratatui::widgets::Borders::ALL).title("Help Page")),
      help_page,
    );
  }
}
