use crate::{
  action::navigate::Action,
  state::tabs::{App, TabPage},
};
use crossterm::event::{Event, KeyCode, KeyModifiers};
use std::error::Error;
pub fn handle_input(app: &mut App) -> std::result::Result<Action, Box<dyn Error>> {
  if let Event::Key(key) = crossterm::event::read()? {
    match key.code {
      KeyCode::Up if app.current_tab == TabPage::HomePage => {
        app.menu.select_prev();
      }
      KeyCode::Down if app.current_tab == TabPage::HomePage => {
        app.menu.select_next();
      }
      KeyCode::Char('q') if key.modifiers == KeyModifiers::CONTROL => return Ok(Action::Quit),

      KeyCode::Char('?') => return Ok(Action::GoToHelpPage),

      KeyCode::Char('h') => return Ok(Action::GoToHomePage),
      _ => {}
    }
  }
  Ok(Action::None)
}
