use super::common::{PromptError, PromptType};
use crate::{io_handl, Color, Modifier};
use std::io::{self, Read, Write};
use termion::{
    clear::CurrentLine,
    cursor::{DetectCursorPos, Goto, Left, Up},
    event::{Event, Key},
    input::Events,
};

pub struct NumberPrompt {
    pub prefix: String,
    pub text: String,
    pub default: Option<String>,
    pub extra: Option<String>,
    pub line: Option<u16>,
    pub prompt_type: PromptType,
}

impl NumberPrompt {
    pub fn write_text<W: Write>(&self, stdout: &mut W) -> io::Result<()> {
        write!(stdout, "{}{}", Left(999), CurrentLine)?;
        write!(
            stdout,
            "{} {} ›",
            Modifier::Bold.a(Color::GreenBright.a(&self.prefix)),
            Modifier::Bold.a(&self.text)
        )?;

        match &self.extra {
            None => {}
            Some(extra) => {
                write!(
                    stdout,
                    "{}({}){} ",
                    Color::BlackBright,
                    extra,
                    Color::BlackBright.get_close()
                )?;
            }
        }

        stdout.flush()?;
        Ok(())
    }

    pub fn write_default<W: Write>(&self, stdout: &mut W) -> io::Result<()> {
        let default = match &self.default {
            Some(d) => d,
            None => return Ok(()),
        };

        write!(stdout, "{}{}", Left(999), CurrentLine)?;
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
                    mouse_pos.1 + 1
                } else {
                    *line + 1
                }
            }
        };

        io_handl!(self.write_text(stdout));
        io_handl!(self.write_default(stdout));

        // true  = +
        // false = -
        let mut sign = true;
        let mut has_dot = false;
        let mut dot_pos: usize = 0;
        let mut pre_data = String::new();
        let mut post_data = String::new();

        let is_signed = match &self.prompt_type {
            PromptType::Float | PromptType::Int => true,
            PromptType::UFloat | PromptType::UInt => false,
            _ => unreachable!(),
        };
        let is_float = match &self.prompt_type {
            PromptType::Float | PromptType::UFloat => true,
            PromptType::Int | PromptType::UInt => false,
            _ => unreachable!(),
        };

        macro_rules! get_data {
            () => {
                format!("{}{}{}", if sign { "" } else { "-" }, pre_data, post_data)
            };
        }

        macro_rules! get_num {
            () => {
                format!("{}{}", pre_data, post_data)
            };
        }

        macro_rules! update {
            () => {
                io_handl!(write!(stdout, "\n{}{}{}", Left(99), CurrentLine, Up(1)));
                io_handl!(self.write_text(stdout));
                io_handl!(write!(stdout, "{}", Color::Cyan.a(get_data!())));
                if post_data.len() >= 1 {
                    io_handl!(write!(stdout, "{}", Left(post_data.len() as u16)));
                }
                io_handl!(stdout.flush());
            };
        }

        macro_rules! send_err {
            ($msg:expr) => {
                io_handl!(write!(stdout, "\n{}{}", Left(9999), CurrentLine));
                io_handl!(write!(
                    stdout,
                    "{}{}{}",
                    CurrentLine,
                    Color::Red.a($msg),
                    Up(1)
                ));
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
                    // If has no value then try to use the default value
                    if get_num!().len() == 0 && self.default.is_some() {
                        post_data = String::new();
                        pre_data = self.default.as_ref().unwrap().to_string();
                        update!();
                    }

                    // If has a dot as last character then remove it
                    let has_dot_as_last = get_num!().len() >= 1
                        && get_num!().chars().nth_back(0) == Option::Some('.');
                    if has_dot_as_last {
                        pre_data = get_num!();
                        post_data = String::new();

                        has_dot = false;
                        pre_data.pop();
                    }

                    if get_num!().len() == 0 {
                        send_err!("The value must has length of 1 or more.");
                        continue;
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
                    // Numbers will push without validator
                    if ch.is_ascii_digit() {
                        pre_data.push(ch);
                        update!();
                    // '-' (minus) key will toggle sign
                    } else if ch == '-' {
                        if !is_signed {
                            send_err!("The number is unsigned, cannot be negative.");
                            continue;
                        }

                        sign = !sign;
                        update!();
                    // '.' (dot) key will put dot or move it
                    // to current pos
                    } else if ch == '.' {
                        if !is_float {
                            send_err!("The number is integer, cannot has dot.");
                            continue;
                        }

                        if has_dot {
                            // The dot is before the cursor position
                            let is_pre_cursor = dot_pos < pre_data.len() + 1;

                            // Split for remove dot
                            let (pre, post) = if is_pre_cursor {
                                pre_data.split_at(dot_pos)
                            } else {
                                post_data.split_at(dot_pos - pre_data.len())
                            };

                            // Remove dot
                            let mut pre = String::from(pre);
                            pre.pop();

                            // Join data
                            if is_pre_cursor {
                                pre_data = format!("{}{}", pre, post);
                            } else {
                                post_data = format!("{}{}", pre, post);
                            }

                            // Put the dot
                            pre_data.push('.');
                            dot_pos = pre_data.len();
                        } else {
                            has_dot = true;

                            pre_data.push('.');
                            dot_pos = pre_data.len();
                        }

                        update!();
                    }
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
                    Option::Some(ch) => {
                        if ch == '.' {
                            has_dot = false;
                        }

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
                    post_data = get_num!();
                    pre_data = String::new();
                    update!();
                }
                Event::Key(Key::End | Key::PageDown) => {
                    pre_data = get_num!();
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
            PromptType::Int | PromptType::UInt | PromptType::Float | PromptType::UFloat => {}
            _ => {
                return Err(PromptError::Custom(String::from(
                    "Prompt struct only can be Int, UInt, Float or UFloat",
                )));
            }
        };

        self.prompt(stdin, stdout, go_back)
    }
}
