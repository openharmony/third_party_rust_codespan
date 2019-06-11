use codespan::LineNumber;
use std::io;
use termcolor::{ColorSpec, WriteColor};

use crate::emitter::Config;

/// The left-hand gutter of a source line.
pub struct Gutter {
    line_number: Option<LineNumber>,
    gutter_padding: usize,
}

impl Gutter {
    pub fn new(line_number: impl Into<Option<LineNumber>>, gutter_padding: usize) -> Gutter {
        Gutter {
            line_number: line_number.into(),
            gutter_padding,
        }
    }

    pub fn emit(&self, writer: &mut impl WriteColor, config: &Config) -> io::Result<()> {
        write!(writer, " ")?;
        match self.line_number {
            None => {
                write!(
                    writer,
                    "{space: >width$}",
                    space = "",
                    width = self.gutter_padding,
                )?;
            },
            Some(line_number) => {
                let line_number_spec = ColorSpec::new()
                    .set_fg(Some(config.line_number_color))
                    .clone();

                writer.set_color(&line_number_spec)?;
                write!(
                    writer,
                    "{line: >width$}",
                    line = line_number,
                    width = self.gutter_padding,
                )?;
                writer.reset()?;
            },
        }
        write!(writer, " ")?;

        Ok(())
    }
}