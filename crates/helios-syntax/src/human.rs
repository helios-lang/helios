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

impl HumanReadableRepr {
    pub fn omitting(&self) -> HumanReadableReprStringBuilder {
        HumanReadableReprStringBuilder {
            article: Some(self.article),
            qualifier: self.qualifier.clone(),
            description: self.description.clone(),
            kind: self.kind.clone(),
            code_repr: self.code_repr.clone(),
            example: self.example.clone(),
        }
    }
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
            write!(f, " (like `{}`)", example)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HumanReadableReprStringBuilder {
    kind: String,
    article: Option<Article>,
    qualifier: Option<String>,
    description: Option<String>,
    code_repr: Option<String>,
    example: Option<String>,
}

impl HumanReadableReprStringBuilder {
    pub fn article(mut self) -> Self {
        self.article = None;
        self
    }

    pub fn qualifier(mut self) -> Self {
        self.qualifier = None;
        self
    }

    pub fn description(mut self) -> Self {
        self.description = None;
        self
    }

    pub fn code_repr(mut self) -> Self {
        self.code_repr = None;
        self
    }

    pub fn example(mut self) -> Self {
        self.example = None;
        self
    }
}

impl Display for HumanReadableReprStringBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(article) = &self.article {
            write!(f, "{} ", article)?;
        }

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
            write!(f, " (like `{}`)", example)?;
        }

        Ok(())
    }
}
