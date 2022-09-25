pub use std::io::{stdin, stdout, Write};

pub use termion::input::MouseTerminal;
pub use termion::raw::IntoRawMode;

pub use crate::macros::{create_prompt, prompt_it};
pub use crate::MyTerminal;
pub use crate::TermRead;

#[macro_export]
macro_rules! create_stdout {
    () => {
        MyTerminal::from(MouseTerminal::from(stdout().into_raw_mode().unwrap()))
    };
}

#[macro_export]
macro_rules! create_events {
    () => {{
        let stdin = ::std::io::stdin();
        $crate::TermRead::events(stdin)
    }};
}

pub use create_events;
pub use create_stdout;
