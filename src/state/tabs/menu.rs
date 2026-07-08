#[allow(dead_code)]
pub struct Menu {
  pub items: Vec<&'static str>,
  pub selected: usize,
}

#[allow(dead_code)]
impl Menu {
  pub fn new() -> Self {
    Menu {
      items: vec!["Continue", "Settings", "Exit"],
      selected: 0,
    }
  }

  pub fn push(&mut self, state: usize) {
    self.selected = state;
  }

  pub fn select_next(&mut self) {
    if self.selected == self.items.len().saturating_sub(1) {
      self.selected = 0;
    } else {
      self.selected = self.selected.saturating_add(1);
    }
  }

  pub fn select_prev(&mut self) {
    if self.selected == 0 {
      self.selected = self.selected.saturating_add(self.items.len());
    }
    self.selected = self.selected.saturating_sub(1);
  }
}
