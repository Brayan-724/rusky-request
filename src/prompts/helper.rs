use std::io::{self, Write};

use crate::themes::FormatTheme;
use crate::PromptType;

#[derive(Clone, Debug)]
pub struct PromptBase {
    pub prefix: String,
    pub text: String,
    pub default: Option<String>,
    pub extra: Option<String>,
    pub line: Option<u16>,
    pub prompt_type: PromptType,
}

impl PromptBase {
    pub fn write_prompt<T: FormatTheme, W: Write>(
        &self,
        theme: &T,
        stdout: &mut W,
    ) -> io::Result<()> {
        write!(stdout, "{}", theme.format_prompt(&self))?;

        stdout.flush()?;
        Ok(())
    }

    pub fn write_default<T: FormatTheme, W: Write>(
        &self,
        theme: &T,
        stdout: &mut W,
    ) -> io::Result<()> {
        write!(stdout, "{}", theme.format_default(&self))?;

        stdout.flush()?;
        Ok(())
    }
}
