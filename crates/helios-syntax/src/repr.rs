use std::fmt::{self, Display};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Article {
    A,
    An,
    The,
}

impl Display for Article {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Article::A => "a",
            Article::An => "an",
            Article::The => "the",
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HumanReadableRepr {
    pub article: Article,
    pub qualifier: Option<String>,
    pub description: Option<String>,
    pub kind: String,
    pub code_repr: Option<String>,
    pub example: Option<String>,
}

impl Display for HumanReadableRepr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.article)?;

        if let Some(qualifier) = &self.qualifier {
            write!(f, "{} ", qualifier)?;
        }

        if let Some(description) = &self.description {
            write!(f, "{} ", description)?;
        }

        write!(f, "{}", self.kind)?;

        if let Some(code_repr) = &self.code_repr {
            write!(f, " (`{}`)", code_repr)?;
        }

        if let Some(example) = &self.example {
            write!(f, " (such as `{}`)", example)?;
        }

        Ok(())
    }
}
