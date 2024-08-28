use crate::style::{Token, TokenType};

/// Tokenize a line of text. This function will return a vector of tokens, where each token is a
/// word, a comment, a string literal, or a function call.
pub fn tokenize(line: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current_word = String::new();
    let mut in_comment = false;
    let mut in_string = false;

    let mut chars = line.chars().peekable();

    while let Some(&c) = chars.peek() {
        if in_comment {
            current_word.push(c);
            chars.next();
        } else if in_string {
            current_word.push(c);
            chars.next();
            if c == '"' {
                in_string = false;
                tokens.push(Token::new(TokenType::StringLiteral, &current_word));
                current_word.clear();
            }
        } else if c.is_ascii_alphanumeric() || c == '_' {
            current_word.push(c);
            chars.next();
        } else {
            if !current_word.is_empty() {
                if chars.peek() == Some(&'(') {
                    tokens.push(Token::new(TokenType::Function, &current_word));
                    current_word.clear();
                } else {
                    tokens.push(classify_token(&current_word));
                    current_word.clear();
                }
            }
            if c == '"' {
                in_string = true;
                current_word.push(c);
                chars.next();
            } else if c == '/' && chars.clone().nth(1) == Some('/') {
                in_comment = true;
                current_word.push(c);
                chars.next();
                current_word.push(chars.next().unwrap());
            } else {
                tokens.push(Token::new(TokenType::Normal, &c.to_string()));
                chars.next();
            }
        }
    }

    if !current_word.is_empty() {
        if in_comment {
            tokens.push(Token::new(TokenType::Comment, &current_word));
        } else if in_string {
            tokens.push(Token::new(TokenType::StringLiteral, &current_word));
        } else {
            tokens.push(classify_token(&current_word));
        }
    }

    tokens
}

/// Classify a token based on its text. If the text is a keyword, return a token with the keyword
/// token type. Otherwise, return a token with the normal token type.
/// # Rust keywords only
fn classify_token(word: &str) -> Token {
    match word {
        "fn" | "let" | "mut" | "if" | "else" | "while" | "for" | "in" | "loop" | "match"
        | "return" => Token::new(TokenType::Keyword, word),
        _ => Token::new(TokenType::Normal, word),
    }
}
