use super::common::{PromptError, PromptType};
use crate::{io_handl, Color, FormatTheme, Modifier, MyFromStr};
use std::fmt::Debug;
use std::io::{self, Read, Write};
use termion::clear::CurrentLine;
use termion::cursor::{DetectCursorPos, Goto, Left};
use termion::event::{Event, Key};
use termion::input::Events;

#[derive(Clone, Debug)]
pub struct Prompt<'a> {
    pub prefix: String,
    pub text: String,
    pub default: Option<String>,
    pub extra: Option<String>,
    pub line: Option<u16>,
    pub prompt_type: PromptType,
    pub theme: &'a dyn FormatTheme,
}

// Instance props
impl<'a> Prompt<'a> {
    pub fn write_text<W: Write>(&self, stdout: &mut W) -> io::Result<()> {
        write!(
            stdout,
            "{}",
            self.theme
                .prompt_text(&self.prefix, &self.text, &self.extra)
        )?;

        stdout.flush()?;
        Ok(())
    }

    pub fn write_default<W: Write>(&self, stdout: &mut W) -> io::Result<()> {
        let default = match &self.default {
            Some(d) => d,
            None => return Ok(()),
        };

        write!(
            stdout,
            "{}[{}]{} ",
            Modifier::Dim,
            default,
            Modifier::Dim.get_close()
        )?;
        stdout.flush()?;
        Ok(())
    }

    pub fn prompt<R: Read, W: Write>(
        &mut self,
        stdin: &mut Events<R>,
        stdout: &mut W,
        go_back: Option<bool>,
    ) -> Result<String, PromptError> {
        let go_back = match go_back {
            Some(v) => v,
            None => true,
        };

        let mouse_pos = io_handl!(DetectCursorPos::cursor_pos(stdout));

        let end_line = match &self.line {
            None => {
                self.line = Option::Some(mouse_pos.1);
                mouse_pos.1 + 1
            }
            Some(line) => {
                io_handl!(write!(stdout, "{}", Goto(0, *line)));
                if go_back {
                    mouse_pos.1
                } else {
                    *line + 1
                }
            }
        };

        io_handl!(self.write_text(stdout));
        io_handl!(self.write_default(stdout));

        let mut pre_data = String::new();
        let mut post_data = String::new();

        macro_rules! get_data {
            () => {
                format!("{}{}", pre_data, post_data)
            };
        }

        macro_rules! update {
            () => {
                io_handl!(self.write_text(stdout));
                io_handl!(write!(stdout, "{}", Color::Cyan.a(get_data!())));
                if post_data.len() >= 1 {
                    io_handl!(write!(stdout, "{}", Left(post_data.len() as u16)));
                }
                io_handl!(stdout.flush());
            };
        }

        for c in stdin {
            let evt = c.unwrap();
            match evt {
                Event::Key(Key::Ctrl('c') | Key::Esc) => {
                    update!();
                    io_handl!(write!(stdout, "{}\n", Color::Red.a("<cancelled>")));
                    io_handl!(stdout.flush());
                    return Err(PromptError::KeyboardInterrupt);
                }
                Event::Key(Key::Char('\n')) => {
                    if get_data!().len() == 0 && self.default.is_some() {
                        post_data = String::new();
                        pre_data = match &self.default {
                            None => unreachable!(),
                            Some(default) => default.clone(),
                        };
                        update!();
                    }

                    io_handl!(write!(
                        stdout,
                        "{}\n{}",
                        Goto(9999, end_line - 1),
                        CurrentLine
                    ));
                    io_handl!(stdout.flush());
                    break;
                }
                Event::Key(Key::Char(ch)) => {
                    pre_data.push(ch);
                    update!();
                }
                Event::Key(Key::Left) => match pre_data.pop() {
                    Option::None => {}
                    Option::Some(ch) => {
                        post_data = format!("{}{}", ch, post_data);
                        update!();
                    }
                },
                Event::Key(Key::Right) => {
                    if post_data.len() >= 1 {
                        let (ch, tail) = post_data.split_at(1);
                        pre_data.push_str(ch);
                        post_data = String::from(tail);
                        update!();
                    } else {
                        pre_data.push_str(post_data.as_str());
                        post_data = String::new();
                        update!();
                    }
                }
                Event::Key(Key::Backspace) => match pre_data.pop() {
                    Option::None => {}
                    Option::Some(_) => {
                        update!();
                    }
                },
                Event::Key(Key::Delete) => {
                    if post_data.len() >= 1 {
                        let (_, tail) = post_data.split_at(1);
                        post_data = String::from(tail);
                        update!();
                    }
                }
                Event::Key(Key::Home | Key::PageUp) => {
                    post_data = get_data!();
                    pre_data = String::new();
                    update!();
                }
                Event::Key(Key::End | Key::PageDown) => {
                    pre_data = get_data!();
                    post_data = String::new();
                    update!();
                }
                _ => {}
            };
        }

        Ok(get_data!())
    }

    pub fn prompt_handled<R: Read, W: Write>(
        &mut self,
        stdin: &mut Events<R>,
        stdout: &mut W,
        go_back: Option<bool>,
    ) -> Result<String, PromptError> {
        match &self.prompt_type {
            PromptType::String | PromptType::Bool => {}
            _ => {
                return Err(PromptError::Custom(String::from(
                    "Prompt struct only can be String, Bool, FilePath or FilePathExisting",
                )));
            }
        };

        macro_rules! send_err {
            ($msg:expr) => {
                io_handl!(write!(stdout, "{}{}", Left(99), CurrentLine));
                io_handl!(write!(stdout, "{}{}", CurrentLine, Color::Red.a($msg)));
            };
        }

        'prompt: loop {
            let prompted = self.prompt(stdin, stdout, go_back.clone());
            match prompted {
                Ok(expr) => match &self.prompt_type {
                    PromptType::String => {
                        if expr.len() == 0 {
                            send_err!("The text should contain 1 character or more");
                            continue 'prompt;
                        }

                        return Ok(expr);
                    }
                    PromptType::Bool => {
                        let parsed = <bool as MyFromStr>::from_str(expr.as_str());
                        if parsed.is_ok() {
                            return Ok(expr);
                        }

                        send_err!("The provided value is invalid. (yes|y|true|no|n|false)");
                        continue 'prompt;
                    }
                    _ => {
                        unreachable!();
                    }
                },
                Err(err) => return Err(err),
            }
        }
    }
}
