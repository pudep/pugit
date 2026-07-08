pub mod menu;
pub mod render;
use std::error::Error;

use ratatui::{
  DefaultTerminal, Frame,
  widgets::{Block, Borders, List},
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
          let mut state = ratatui::widgets::ListState::default();
          state.select(Some(self.menu.selected));
          terminal.draw(|frame| {
            let list = List::new(self.menu.items.iter().copied())
              .block(Block::new().borders(Borders::ALL).title("Menu"))
              .highlight_style(ratatui::style::Style::default().bg(ratatui::style::Color::Cyan))
              .highlight_symbol("> ");
            frame.render_stateful_widget(list, frame.area(), &mut state);
          })?;
        }
        TabPage::HelpPage => {
          terminal.draw(|frame| self.render(frame))?;
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
