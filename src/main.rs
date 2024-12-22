mod editor;

use crossterm::terminal;

fn main() -> crossterm::Result<()> {
    terminal::enable_raw_mode()?;
    let mut editor = editor::Editor::new();
    while editor.run()? {}
    Ok(())
}