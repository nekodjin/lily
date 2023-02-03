use logos::{Logos, Lexer};

#[derive(Debug, Logos, PartialEq, Eq, Clone, Hash)]
enum Token {
    #[error]
    #[regex(r"[ \t\r]", logos::skip)]
    #[regex(r";[^\n]*", logos::skip)]
    Error,

    #[token("\n")]
    LineBreak,
}
