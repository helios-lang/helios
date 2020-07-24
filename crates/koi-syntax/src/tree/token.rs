pub struct TokenData {
    kind: TokenKind,
    leading_trivia: Vec<Trivia>,
    trailing_trivia: Vec<Trivia>,
}

pub enum TokenKind {
    Identifier
}

pub enum Trivia {
    Tab(usize),
    Space(usize),
    LineFeed(usize),
    CarriageReturn(usize),
    CarriageReturnLineFeed(usize),
}
