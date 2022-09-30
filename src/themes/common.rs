use std::fmt::Debug;

use rusky_icons::icon_sm_right_arrow;
use termion::clear::CurrentLine;
use termion::cursor::Left;

use crate::{Color, Modifier, PromptBase};

pub trait FormatTheme: Debug {
    fn format_prompt(&self, prompt: &PromptBase) -> String {
        let mut out = format!(
            "{}{}{} {} {} ",
            Left(999),
            CurrentLine,
            Modifier::Bold.a(Color::GreenBright.a(&prompt.prefix)),
            Modifier::Bold.a(&prompt.text),
            Modifier::Dim.a(icon_sm_right_arrow::STR)
        );

        if prompt.extra.is_some() {
            let extra = prompt.extra.as_ref().unwrap();
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

    fn format_default(&self, prompt: &PromptBase) -> String {
        let default = match &prompt.default {
            Some(d) => d,
            None => return String::new(),
        };

        format!(
            "{}[{}]{} ",
            Modifier::Dim,
            default,
            Modifier::Dim.get_close()
        )
    }
}
