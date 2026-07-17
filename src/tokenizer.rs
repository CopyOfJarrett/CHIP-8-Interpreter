use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct Tokenizer<'src> {
    src: &'src str,
    pos: usize,
}

impl<'src> Tokenizer<'src> {
    pub const fn new(src: &'src str) -> Self {
        Self { src, pos: 0 }
    }

    pub fn tokenize(&mut self) -> Vec<Token<'src>> {
        let mut tokens = Vec::new();
        loop {
            match self.advance_token() {
                token if matches!(token.kind, TokenKind::Eof) => {
                    tokens.push(token);
                    break;
                }
                token => tokens.push(token),
            }
        }
        tokens
    }

    fn advance_token(&mut self) -> Token<'src> {
        let start = self.pos;

        let Some(first) = self.bump() else {
            return self.build_simple_token(TokenKind::Eof, start);
        };

        match first {
            b'(' => self.build_simple_token(TokenKind::LeftParen, start),
            b')' => self.build_simple_token(TokenKind::RightParen, start),
            b'{' => self.build_simple_token(TokenKind::LeftBrace, start),
            b'}' => self.build_simple_token(TokenKind::RightBrace, start),
            _ => todo!(),
        }
    }

    fn build_simple_token(&self, kind: TokenKind, start: usize) -> Token<'src> {
        Token::new(kind, &self.src[start..self.pos], None)
    }

    fn bump(&mut self) -> Option<u8> {
        let b = self.peek()?;
        self.pos += 1;
        Some(b)
    }

    fn peek(&self) -> Option<u8> {
        self.src.as_bytes().get(self.pos).copied()
    }
}
