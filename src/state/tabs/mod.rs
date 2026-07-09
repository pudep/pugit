pub mod draw;
pub mod menu;
use std::error::Error;

use ratatui::{
  DefaultTerminal,
};

use crate::state::tabs::menu::Menu;
use crate::{action::navigate::Action, state::tabs::TabPage::HomePage};

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum TabPage {
  HomePage,
  HelpPage,
}

#[allow(dead_code)]
pub struct App {
  pub current_tab: TabPage,
  pub menu: Menu,
  pub text: String,
}

#[allow(dead_code)]
impl App {
  pub fn new() -> App {
    App {
      current_tab: HomePage,
      menu: Menu::new(),
      text: "".to_string(),
    }
  }

  fn draw(&mut self, terminal: &mut DefaultTerminal) -> std::result::Result<(), Box<dyn Error>> {
    loop {
      match self.current_tab {
        TabPage::HomePage => {
          self.draw_home_page(terminal)?;
        }
        TabPage::HelpPage => {
          self.draw_help_page(terminal)?;
        }
      }
      match crate::keys::init::handle_input(self)? {
        Action::Quit => break,
        Action::GoToHomePage => {
          self.current_tab = TabPage::HomePage;
        }
        Action::GoToHelpPage => {
          self.current_tab = TabPage::HelpPage;
        }
        Action::None => continue,
      }
    }
    Ok(())
  }


  pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    ratatui::run(|terminal| self.draw(terminal))?;
    Ok(())
  }
}
