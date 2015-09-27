extern crate scribe;
extern crate rustbox;
extern crate pad;

pub mod models;
pub mod view;
mod input;
mod commands;

use models::application::Mode;
use view::{Data, StatusLine};
use rustbox::Color;
use scribe::buffer::Position;

fn main() {
    let mut application = models::application::new();
    let terminal = models::terminal::new();
    let mut buffer_presenter = view::presenters::buffer::new(terminal.height()-1);

    loop {
        // Draw the current application state to the screen.
        let view_data = match application.workspace.current_buffer() {
            Some(ref mut buffer) => buffer_presenter.data(buffer, &mut application.mode),
            None => Data{
                tokens: Vec::new(),
                cursor: Position{ line: 0, offset: 0 },
                highlight: None,
                status_line: StatusLine{
                    content: "".to_string(),
                    color: Color::Default
                }
            },
        };
        match application.mode {
            Mode::Open(ref mode) => view::modes::open::display(&terminal, &view_data, mode),
            _ => view::modes::default::display(&terminal, &view_data),
        }

        match terminal.get_input() {
            Some(key) => {
                // Pass the input to the current mode.
                let command = match application.mode {
                    Mode::Normal => input::modes::normal::handle(key),
                    Mode::Insert(ref mut i) => input::modes::insert::handle(i, key),
                    Mode::Jump(ref mut j) => input::modes::jump::handle(j, key),
                    Mode::Open(ref mut o) => input::modes::open::handle(o, key),
                    Mode::Select(_) => input::modes::select::handle(key),
                    Mode::Exit => break,
                };

                // If the current mode returned a command, run it.
                match command {
                    Some(c) => c(&mut application),
                    None => (),
                }

                // Check if the command resulted in an exit, before
                // looping again and asking for input we won't use.
                match application.mode {
                    Mode::Exit => break,
                    _ => {}
                }
            },
            None => {},
        }
    }
}
