use colored::*;
use std::fmt::{self, Display};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FormattedStringSegment {
    LineBreak,
    Text(String),
    Code(String),
    CodeBlock(String),
    List(Vec<FormattedString>),
}

impl FormattedStringSegment {
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text(text.into())
    }

    pub fn code(code: impl Into<String>) -> Self {
        Self::Code(code.into())
    }

    pub fn code_block(code_block: impl Into<String>) -> Self {
        Self::CodeBlock(code_block.into())
    }

    pub fn list(list: impl Into<Vec<FormattedString>>) -> Self {
        Self::List(list.into())
    }
}

impl Display for FormattedStringSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let colorize = colored::control::SHOULD_COLORIZE.should_colorize();
        match self {
            Self::LineBreak => write!(f, "\n\n"),
            Self::Text(text) => write!(f, "{text}"),
            Self::Code(code) => {
                if colorize {
                    write!(f, "{}", code.yellow())
                } else {
                    write!(f, "`{code}`")
                }
            }
            Self::CodeBlock(block) => {
                if colorize {
                    write!(f, "    {}", block.yellow())
                } else {
                    write!(f, "    {block}")
                }
            }
            Self::List(lines) => {
                for line in lines {
                    writeln!(f, "    {line}")?;
                }
                Ok(())
            }
        }
    }
}

impl From<String> for FormattedStringSegment {
    fn from(string: String) -> Self {
        FormattedStringSegment::Text(string)
    }
}

impl From<&str> for FormattedStringSegment {
    fn from(string: &str) -> Self {
        FormattedStringSegment::Text(string.to_string())
    }
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct FormattedString {
    segments: Vec<FormattedStringSegment>,
}

impl FormattedString {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, segment: impl Into<FormattedStringSegment>) {
        self.segments.push(segment.into())
    }

    pub fn with(mut self, segment: impl Into<FormattedStringSegment>) -> Self {
        self.segments.push(segment.into());
        self
    }

    pub fn line_break(self) -> Self {
        self.with(FormattedStringSegment::LineBreak)
    }

    pub fn text(self, text: impl Into<String>) -> Self {
        self.with(FormattedStringSegment::text(text))
    }

    pub fn code(self, code: impl Into<String>) -> Self {
        self.with(FormattedStringSegment::code(code))
    }

    pub fn code_block(self, code_block: impl Into<String>) -> Self {
        self.with(FormattedStringSegment::LineBreak)
            .with(FormattedStringSegment::code_block(code_block))
            .with(FormattedStringSegment::LineBreak)
    }

    pub fn list(self, list: impl Into<Vec<FormattedString>>) -> Self {
        self.with(FormattedStringSegment::LineBreak)
            .with(FormattedStringSegment::list(list))
            .with(FormattedStringSegment::LineBreak)
    }

    pub fn finish(self) -> String {
        self.to_string().trim_end().to_string()
    }
}

impl Display for FormattedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for segment in &self.segments {
            write!(f, "{}", segment)?;
        }

        Ok(())
    }
}

impl From<String> for FormattedString {
    fn from(s: String) -> Self {
        Self::new().with(s)
    }
}

impl From<&str> for FormattedString {
    fn from(s: &str) -> Self {
        Self::new().with(s)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_formatter() {
//         println!();
//         let text = FormattedString::new()
//             .text("The 2nd argument for the function ")
//             .code("foo")
//             .text(" expected a value of type:")
//             .code_block("Vector Char")
//             .text("But I received a value of type:")
//             .code_block("String")
//             .text("Here's a list of random types:")
//             .list(
//                 ["Foo.T", "Foo.Bar.T", "Foo.Bar.Baz.T", "Quux.T"]
//                     .iter()
//                     .map(|s| FormattedString::new().code(*s).text(" (info)"))
//                     .collect::<Vec<_>>(),
//             )
//             .finish();
//
//         println!("{text}\n");
//     }
// }
