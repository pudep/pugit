use ratatui::{DefaultTerminal, Frame, layout::Constraint, widgets::{Block, Borders, Paragraph}};

pub fn app(terminal: &mut DefaultTerminal)-> std::result::Result<(), Box<dyn std::error::Error>> {
  loop {
    terminal.draw(render)?;
    if crate::keys::init::main()? {
      break;
    }
  }
  Ok(())
}

fn render(frame: &mut Frame) {
  let [buffer] = ratatui::layout::Layout::vertical([Constraint::Fill(1)]).areas(frame.area());

  frame.render_widget(Paragraph::new("Hello, World!\nThis is a Git wrapper!").block(Block::new().borders(Borders::NONE)), buffer);
}
