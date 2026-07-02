use crossterm::event::{Event, KeyCode, KeyModifiers};
use ratatui::{DefaultTerminal, Frame, layout::Constraint, widgets::{Block, Borders, Paragraph}};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
  ratatui::run(app)?;
  Ok(())
}

fn app(terminal: &mut DefaultTerminal)-> std::result::Result<(), Box<dyn std::error::Error>> {
  loop {
    terminal.draw(render)?;
    if let Event::Key(key) = crossterm::event::read()? {
      match key.code {
        KeyCode::Char('q') if key.modifiers == KeyModifiers::CONTROL => break,
         _ => {},
      }
    }
  }
  Ok(())
}

fn render(frame: &mut Frame) {
  let [buffer] = ratatui::layout::Layout::vertical([Constraint::Fill(1)]).areas(frame.area());

  frame.render_widget(Paragraph::new("Hello, World!\nThis is a Git wrapper!").block(Block::new().borders(Borders::NONE)), buffer);
}
