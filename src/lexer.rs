use crate::token::{Token, TokenKind};
use thiserror::Error;

#[derive(Debug)]
pub struct Lexer<'src> {
    src: &'src str,
    position: usize,
    line: usize,
    is_done: bool,
}

impl<'src> Lexer<'src> {
    pub const fn new(src: &'src str) -> Self {
        Self {
            src,
            position: 0,
            line: 1,
            is_done: false,
        }
    }

    fn advance_token(&mut self) -> Result<Token<'src>, LexError> {
        let start = self.position;

        let Some(first_byte) = self.bump() else {
            return Ok(self.build_simple_token(TokenKind::Eof, start));
        };

        match first_byte {
            b'(' => Ok(self.build_simple_token(TokenKind::LeftParen, start)),
            b')' => Ok(self.build_simple_token(TokenKind::RightParen, start)),
            b'{' => Ok(self.build_simple_token(TokenKind::LeftBrace, start)),
            b'}' => Ok(self.build_simple_token(TokenKind::RightBrace, start)),
            b',' => Ok(self.build_simple_token(TokenKind::Comma, start)),
            b'.' => Ok(self.build_simple_token(TokenKind::Dot, start)),
            b'-' => Ok(self.build_simple_token(TokenKind::Minus, start)),
            b'+' => Ok(self.build_simple_token(TokenKind::Plus, start)),
            b';' => Ok(self.build_simple_token(TokenKind::Semicolon, start)),
            b'/' => Ok(self.build_simple_token(TokenKind::Slash, start)),
            b'*' => Ok(self.build_simple_token(TokenKind::Star, start)),
            _ => Err(LexError::UnexpectedCharacter {
                line: self.line,
                unexpected: first_byte as char,
            }),
        }
    }

    fn build_simple_token(&self, kind: TokenKind, start: usize) -> Token<'src> {
        Token::new(kind, &self.src[start..self.position], None)
    }

    fn bump(&mut self) -> Option<u8> {
        let b = self.peek()?;
        self.position += 1;
        Some(b)
    }

    fn peek(&self) -> Option<u8> {
        self.src.as_bytes().get(self.position).copied()
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Result<Token<'src>, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }

        let token = self.advance_token();

        if matches!(&token, Ok(t) if t.kind == TokenKind::Eof) {
            self.is_done = true;
        }

        Some(token)
    }
}

#[derive(Error, Debug)]
pub enum LexError {
    #[error("[line {line}] Error: Unexpected character: {unexpected}")]
    UnexpectedCharacter { line: usize, unexpected: char },
}
