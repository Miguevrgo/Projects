#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Keyword,
    Comment,
    Function,
    StringLiteral,
    Normal,
}

pub struct Token {
    pub token_type: TokenType,
    pub text: String,
}

impl Token {
    pub fn new(token_type: TokenType, text: &str) -> Self {
        Token {
            token_type,
            text: text.to_string(),
        }
    }
}
