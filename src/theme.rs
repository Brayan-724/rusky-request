use crate::{Color, Modifier};
use rusky_icons::icon_sm_right_arrow;
use std::fmt::Debug;
use termion::clear::CurrentLine;
use termion::cursor::Left;

pub trait FormatTheme: Debug {
    fn prompt_text(&self, prefix: &String, text: &String, extra: &Option<String>) -> String {
        let mut out = format!(
            "{}{}{} {} {} ",
            Left(999),
            CurrentLine,
            Modifier::Bold.a(Color::GreenBright.a(prefix)),
            Modifier::Bold.a(text),
            Modifier::Dim.a(icon_sm_right_arrow::STR)
        );

        if extra.is_some() {
            let extra = extra.as_ref().unwrap();
            let extra = format!(
                "{}({}){} ",
                Color::BlackBright,
                extra,
                Color::BlackBright.get_close()
            );
            out = format!("{}{}", out, extra);
        }

        out
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefaultTheme;

impl FormatTheme for DefaultTheme {}
