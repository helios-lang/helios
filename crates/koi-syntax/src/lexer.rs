#![allow(dead_code)]

use crate::source::Cursor;
use crate::syntax::SyntaxKind;
use unicode_xid::UnicodeXID;

/// Checks if the given character is a valid start of an identifier. A valid
/// start of an identifier is any Unicode code point that satisfies the
/// `XID_Start` property.
fn is_identifier_start(c: char) -> bool {
    // Fast-path for ASCII identifiers
    ('a' <= c && c <= 'z')
        || ('A' <= c && c <= 'Z')
        || c == '_'
        || c.is_xid_start()
}

/// Checks if the given character is a valid continuation of an identifier.
/// A valid continuation of an identifier is any Unicode code point that
/// satisfies the `XID_Continue` property.
fn is_identifier_continue(c: char) -> bool {
    // Fast-path for ASCII identifiers
    ('a' <= c && c <= 'z')
        || ('A' <= c && c <= 'Z')
        || ('0' <= c && c <= '9')
        || c == '_'
        || c.is_xid_continue()
}

/// Checks if the given character is a grouping delimiter.
fn is_grouping_delimiter(c: char) -> bool {
    matches!(c, '{' | '}' | '[' | ']' | '(' | ')')
}

/// Checks if the given character is a recognised symbol.
#[rustfmt::skip]
fn is_symbol(c: char) -> bool {
    match c {
        '&' | '*' | '@' | '!' | '^' | ':' | ',' | '$' | '.' | '–' | '—' | '=' |
        '-' | '%' | '+' | '#' | '?' | ';' | '£' | '~' | '|' | '/' | '\\'| '<' |
        '>' | '{' | '}' | '[' | ']' | '(' | ')' => true,
        _ => false,
    }
}

fn is_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\r' | '\n')
}

#[derive(Clone, Debug, PartialEq)]
pub enum LexerMode {
    Normal,
    Grouping,
}

impl Default for LexerMode {
    fn default() -> Self {
        Self::Normal
    }
}

pub struct Lexer {
    cursor: Cursor,
    consumed_chars: Vec<char>,
    mode_stack: Vec<LexerMode>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            cursor: Cursor::new(source),
            consumed_chars: Vec::new(),
            mode_stack: vec![LexerMode::Normal],
        }
    }

    #[allow(dead_code)]
    pub(crate) fn push_mode(&mut self, mode: LexerMode) {
        self.mode_stack.push(mode);
    }

    #[allow(dead_code)]
    pub(crate) fn pop_mode(&mut self) -> Option<LexerMode> {
        self.mode_stack.pop()
    }

    fn current_mode(&self) -> LexerMode {
        self.mode_stack.last().cloned().unwrap_or_default()
    }
}

impl Lexer {
    /// Retrieves the next character in the iterator.
    fn next_char(&mut self) -> Option<char> {
        self.cursor.advance().map(|c| {
            self.consumed_chars.push(c);
            c
        })
    }

    /// Peeks the next character without consuming it.
    fn peek(&self) -> char {
        self.peek_at(0)
    }

    /// Peeks the character at the given index without consuming it.
    fn peek_at(&self, n: usize) -> char {
        self.cursor.nth(n)
    }

    /// Checks if the `Lexer` has reached the end of the input.
    pub(crate) fn is_at_end(&self) -> bool {
        self.cursor.source_len() == 0
    }

    pub(crate) fn current_pos(&self) -> usize {
        self.cursor.pos
    }

    /// Attempts to consume the next character if it matches the provided
    /// character `c`. Returns a `bool` indicating if it was successful or not.
    fn consume(&mut self, c: char) -> bool {
        if self.peek() == c {
            self.next_char();
            true
        } else {
            false
        }
    }

    /// Consumes the input while the given `predicate` holds true. Returns the
    /// count of characters traversed.
    fn consume_while<F>(&mut self, predicate: F) -> usize
    where
        F: Fn(char) -> bool,
    {
        let mut consumed = 0;
        while predicate(self.peek()) && !self.is_at_end() {
            self.next_char();
            consumed += 1;
        }
        consumed
    }

    /// Consumes the input while the given `predicate` holds true, building a
    /// `Vec<char>` for all the characters consumed.
    fn consume_build<F>(&mut self, predicate: F) -> Vec<char>
    where
        F: Fn(char) -> bool,
    {
        let mut vec = Vec::new();
        while predicate(self.peek()) && !self.is_at_end() {
            if let Some(c) = self.next_char() {
                vec.push(c);
            }
        }
        vec
    }
}

impl Lexer {
    fn tokenize_normal(&mut self) -> Option<SyntaxKind> {
        let kind = match self.next_char()? {
            c if is_whitespace(c) => self.lex_whitespace(c),
            c if is_symbol(c) => self.lex_symbol(c),
            c if is_identifier_start(c) => self.lex_identifier(c),
            c => todo!("Lexer::tokenize_normal({:?})", c),
        };

        Some(kind)
    }

    fn lex_whitespace(&mut self, _: char) -> SyntaxKind {
        self.consume_while(is_whitespace);
        SyntaxKind::Whitespace
    }

    /// Matches any character that is a valid symbol.
    ///
    /// _TODO:_ Perhaps we could handle cases with confused symbols, such as
    /// U+037E, the Greek question mark, which looks like a semicolon (compare
    /// ';' with ';').
    fn lex_symbol(&mut self, symbol: char) -> SyntaxKind {
        match symbol {
            '?' => {
                if (self.peek(), self.peek_at(1)) == ('?', '?') {
                    // Consume the next two question marks
                    self.next_char();
                    self.next_char();
                    SyntaxKind::Kwd_Unimplemented
                } else {
                    SyntaxKind::Sym_Question
                }
            }
            _ => {
                if let Some(symbol) =
                    SyntaxKind::symbol_from_chars(symbol, self.peek())
                {
                    self.next_char();
                    symbol
                } else {
                    SyntaxKind::symbol_from_char(symbol)
                }
            }
        }
    }

    /// Matches every character that can be part of an identifier. This includes
    /// upper and lower-case letters, decimal digits and the underscore.
    fn lex_identifier(&mut self, first_char: char) -> SyntaxKind {
        let rest = self.consume_build(is_identifier_continue);
        let vec = [&vec![first_char], &rest[..]].concat();
        let string: String = vec.into_iter().collect();
        self.lex_keyword_or_identifier(string)
    }

    /// Attempts to match the provided `string` to a keyword, returning a
    /// `TokenKind::Keyword` if a match is found, otherwise a
    /// `TokenKind::Identifier`.
    #[rustfmt::skip]
    fn lex_keyword_or_identifier(&mut self, string: String) -> SyntaxKind {
        match &*string {
            "alias"     => SyntaxKind::Kwd_Alias,
            "and"       => SyntaxKind::Kwd_And,
            "as"        => SyntaxKind::Kwd_As,
            "const"     => SyntaxKind::Kwd_Const,
            "else"      => SyntaxKind::Kwd_Else,
            "extend"    => SyntaxKind::Kwd_Extend,
            "external"  => SyntaxKind::Kwd_External,
            "for"       => SyntaxKind::Kwd_For,
            "function"  => SyntaxKind::Kwd_Function,
            "if"        => SyntaxKind::Kwd_If,
            "import"    => SyntaxKind::Kwd_Import,
            "in"        => SyntaxKind::Kwd_In,
            "internal"  => SyntaxKind::Kwd_Internal,
            "let"       => SyntaxKind::Kwd_Let,
            "match"     => SyntaxKind::Kwd_Match,
            "module"    => SyntaxKind::Kwd_Module,
            "not"       => SyntaxKind::Kwd_Not,
            "of"        => SyntaxKind::Kwd_Of,
            "or"        => SyntaxKind::Kwd_Or,
            "public"    => SyntaxKind::Kwd_Public,
            "ref"       => SyntaxKind::Kwd_Ref,
            "return"    => SyntaxKind::Kwd_Return,
            "take"      => SyntaxKind::Kwd_Take,
            "type"      => SyntaxKind::Kwd_Type,
            "var"       => SyntaxKind::Kwd_Var,
            "where"     => SyntaxKind::Kwd_Where,
            "while"     => SyntaxKind::Kwd_While,
            "with"      => SyntaxKind::Kwd_With,
            _           => SyntaxKind::Identifier,
        }
    }
}

impl Iterator for Lexer {
    type Item = SyntaxKind;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_mode() {
            LexerMode::Normal => self.tokenize_normal(),
            LexerMode::Grouping => todo!("LexerMode::Grouping"),
        }
    }
}

#[test]
fn test_lexer() {
    let source = "function add(x: Int, y: Int): Int = x + y";
    let lexer = Lexer::new(source.to_string());
    let tokens = lexer.collect::<Vec<_>>();
    println!("{:?}", tokens);
}
