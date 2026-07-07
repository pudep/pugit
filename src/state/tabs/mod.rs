use crate::state::tabs::TabPage::HomePage;

#[allow(dead_code)]
pub enum TabPage {
  HomePage, 
  HelpPage,
}

#[allow(dead_code)]
pub struct App {
  pub current_tab: TabPage,
  pub render_new: bool,
  pub text: String,
}

#[allow(dead_code)]
impl App {
  fn new() -> App {
    App {
      current_tab: HomePage,
      render_new: true,
      text: "".to_string(),
    }
  }

  fn render(&self){
    if self.render_new {

    }
  }
}
