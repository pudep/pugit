pub mod render;
use std::error::Error;

use ratatui::{DefaultTerminal, Frame};

use crate::{action::navigate::Action, state::tabs::TabPage::HomePage};

#[allow(dead_code)]
pub enum TabPage {
  HomePage,
  HelpPage,
}

#[allow(dead_code)]
pub struct App {
  pub current_tab: TabPage,
  pub text: String,
}

#[allow(dead_code)]
impl App {
  pub fn new() -> App {
    App {
      current_tab: HomePage,
      text: "".to_string(),
    }
  }

  fn draw(&mut self, terminal: &mut DefaultTerminal) -> std::result::Result<(), Box<dyn Error>> {
    loop {
      terminal.draw(|frame| self.render(frame))?;
      match crate::keys::init::handle_input()? {
        Action::Quit => break,
        Action::GoToHomePage => {
          self.current_tab = TabPage::HomePage;
        },
        Action::GoToHelpPage => {
          self.current_tab = TabPage::HelpPage;
        }
        Action::None => continue,
      }
    }
    Ok(())
  }

  fn render(&self, frame: &mut Frame) {
    match self.current_tab {
      TabPage::HomePage => {
        App::render_homepage(frame);
      }
      TabPage::HelpPage => {
        App::render_help_page(frame); 
      }
    }
  }

  pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    ratatui::run(|terminal| self.draw(terminal))?;
    Ok(())
  }
}
