use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::{event, terminal};
use std::time::Duration;
use std::result::Result;
struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable raw mode")
    }
}

struct Reader;

impl Reader {
    fn read_key(&self) -> Result<KeyEvent, Box<dyn std::error::Error>> {
        loop {
            if event::poll(Duration::from_millis(500))? {
                if let Event::Key(event) = event::read()? {
                    return Ok(event);
                }
            }
        }
    }
}

struct Editor {
    reader: Reader,
}

impl Editor {
    fn new() -> Self {
        Self { reader: Reader }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    loop {
        if event::poll(Duration::from_millis(1000))? {
            if let Event::Key(event) = event::read()? {
                match event {
                    KeyEvent {
                        code: KeyCode::Char('q'),
                        modifiers: event::KeyModifiers::CONTROL,
                        ..
                    } => break,
                    _ => {
                        //todo
                    }
                }
                println!("{:?}\r", event);
            };
        } else {
            //lL
            println!("No input yet\r");
        }
    }
    Ok(())
}