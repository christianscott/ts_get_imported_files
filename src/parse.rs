use crate::dependency::Dependency;
use crate::token::{Token, TokenKind};

pub fn parse(tokens: Vec<Token>) -> Vec<Dependency> {
    Parser { tokens, current: 0 }.parse()
}

pub struct ParseErr {
    token: Token,
    message: String,
}

#[derive(Debug)]
struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

macro_rules! check {
    ($self:ident, $($p:pat),+) => {
        match $self.peek() {
            $(Token { kind: $p, .. }) |+ => true,
            _ => false,
        }
    };
}

macro_rules! eat {
    ($self:ident, $($p:pat),+) => {
        match $self.peek() {
            $(Token { kind: $p, .. }) |+ => Some($self.advance()),
            _ => None,
        }
    };
}

macro_rules! did_eat {
    ($self:ident, $($p:pat),+) => {
        match $self.peek() {
            $(Token { kind: $p, .. }) |+ => { $self.advance(); true },
            _ => false,
        }
    };
}

macro_rules! consume {
    ($self:ident, $p:pat, $message:literal) => {
        if let Some(tok) = eat!($self, $p) {
            Ok(tok)
        } else {
            Err(ParseErr {
                message: $message.to_string(),
                token: $self.peek(),
            })
        }
    };
}

impl Parser {
    fn parse(&mut self) -> Vec<Dependency> {
        let mut deps = Vec::new();

        while !self.is_at_end() {
            match self.import_or_export() {
                Ok(dep) => deps.push(dep),
                Err(ParseErr { message, token }) => {
                    println!("parse error at {:?}: {}", token.kind, message)
                }
            }
        }

        deps
    }

    fn import_or_export(&mut self) -> Result<Dependency, ParseErr> {
        if did_eat!(self, TokenKind::Import) {
            self.import()
        } else {
            self.export()
        }
    }

    fn import(&mut self) -> Result<Dependency, ParseErr> {
        if did_eat!(self, TokenKind::LeftParen) {
            self.dynamic_import()
        } else {
            self.default_import()
        }
    }

    fn dynamic_import(&mut self) -> Result<Dependency, ParseErr> {
        let path = consume!(self, TokenKind::Str(_), "Expect a string")?;
        consume!(
            self,
            TokenKind::RightParen,
            "Expect ) after dynamic import."
        )?;
        // TODO: eat semicolon here?
        Ok(Dependency { path })
    }

    fn default_import(&mut self) -> Result<Dependency, ParseErr> {
        if did_eat!(self, TokenKind::Identifier) {
            consume!(self, TokenKind::Frm, "Expect 'from' keyword.")?;
            let path = consume!(self, TokenKind::Str(_), "Expect a string.")?;
            // TODO: semicolon?
            Ok(Dependency { path })
        } else {
            self.destructured_import()
        }
    }

    fn destructured_import(&mut self) -> Result<Dependency, ParseErr> {
        if did_eat!(self, TokenKind::LeftBrace) {
            while !check!(self, TokenKind::RightBrace) && !self.is_at_end() {
                // skip over symbols, we don't really care what's going on here
                self.advance();
            }
            consume!(
                self,
                TokenKind::RightBrace,
                "Expect } after destructured import symbols."
            )?;
            consume!(self, TokenKind::Frm, "Expect 'from' after destructure.")?;
            Ok(Dependency {
                path: consume!(self, TokenKind::Str(_), "Expect a string.")?,
            })
        } else {
            self.namespace_import()
        }
    }

    fn namespace_import(&mut self) -> Result<Dependency, ParseErr> {
        if did_eat!(self, TokenKind::Star) {
            consume!(self, TokenKind::As, "Expect 'as' after '*'.")?;
            consume!(self, TokenKind::Identifier, "Expect identifier after 'as'.")?;
            consume!(self, TokenKind::Frm, "Expect 'from' after identifier.")?;
            Ok(Dependency {
                path: consume!(self, TokenKind::Str(_), "Expect a string.")?,
            })
        } else {
            Err(ParseErr {
                token: self.peek(),
                message: "Expect dynamic, default, destructured, or namespace import after 'import' keyword".to_string()
            })
        }
    }

    fn export(&mut self) -> Result<Dependency, ParseErr> {
        if did_eat!(self, TokenKind::Export) {
            self.destructured_export()
        } else {
            unreachable!()
        }
    }

    fn destructured_export(&mut self) -> Result<Dependency, ParseErr> {
        if did_eat!(self, TokenKind::LeftBrace) {
            while !check!(self, TokenKind::RightBrace) && !self.is_at_end() {
                // skip over symbols, we don't really care what's going on here
                self.advance();
            }
            consume!(
                self,
                TokenKind::RightBrace,
                "Expect } after destructured exprt symbols."
            )?;
            consume!(self, TokenKind::Frm, "Expect 'from' after destructure.")?;
            Ok(Dependency {
                path: consume!(self, TokenKind::Str(_), "Expect a string.")?,
            })
        } else {
            self.namespace_export()
        }
    }

    fn namespace_export(&mut self) -> Result<Dependency, ParseErr> {
        if did_eat!(self, TokenKind::Star) {
            consume!(self, TokenKind::As, "Expect 'as' after '*'.")?;
            consume!(self, TokenKind::Identifier, "Expect identifier after 'as'.")?;
            consume!(self, TokenKind::Frm, "Expect 'from' after identifier.")?;
            Ok(Dependency {
                path: consume!(self, TokenKind::Str(_), "Expect a string.")?,
            })
        } else {
            unreachable!()
        }
    }

    // Utilities

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        if self.current >= self.tokens.len() {
            return true;
        }

        if let Token {
            kind: TokenKind::Eof,
            ..
        } = self.peek()
        {
            true
        } else {
            false
        }
    }

    fn peek(&self) -> Token {
        self.peek_nth(0)
    }

    fn previous(&self) -> Token {
        self.peek_nth(-1)
    }

    fn peek_nth(&self, n: i16) -> Token {
        self.tokens[((self.current as i16) + n) as usize].clone() // TODO: avoidable??
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::token::{Range, Source, Token, TokenKind, TokenKind::*};
    use std::rc::Rc;

    fn new_token_factory() -> impl Fn(TokenKind) -> Token {
        let source = Rc::new(Source::new("For testing".to_string(), vec![]));
        move |kind| Token {
            kind,
            line: 1,
            span: Range(0, 0),
            source: Rc::clone(&source),
        }
    }

    #[test]
    fn test_dynamic_import() {
        let token = new_token_factory();
        let path = token(Str("./module".to_string()));
        assert_eq!(
            parse(vec![
                token(Import),
                token(LeftParen),
                path.clone(),
                token(RightParen),
            ]),
            vec![Dependency { path }],
        );
    }

    #[test]
    fn test_default_import() {
        let token = new_token_factory();
        let path = token(Str("./module".to_string()));
        assert_eq!(
            parse(vec![
                token(Import),
                token(Identifier),
                token(Frm),
                path.clone(),
            ]),
            vec![Dependency { path }],
        );
    }

    #[test]
    fn test_destructured_import() {
        let token = new_token_factory();
        let path = token(Str("./module".to_string()));
        assert_eq!(
            parse(vec![
                token(Import),
                token(LeftBrace),
                token(Identifier),
                token(Comma),
                token(Identifier),
                token(RightBrace),
                token(Frm),
                path.clone(),
            ]),
            vec![Dependency { path }],
        );
    }

    #[test]
    fn test_destructured_import_trailing_comma() {
        let token = new_token_factory();
        let path = token(Str("./module".to_string()));
        assert_eq!(
            parse(vec![
                token(Import),
                token(LeftBrace),
                token(Identifier),
                token(Comma),
                token(Identifier),
                token(Comma),
                token(RightBrace),
                token(Frm),
                path.clone(),
            ]),
            vec![Dependency { path }],
        );
    }

    #[test]
    fn test_namespace_import() {
        let token = new_token_factory();
        let path = token(Str("./module".to_string()));
        assert_eq!(
            parse(vec![
                token(Import),
                token(Star),
                token(As),
                token(Identifier),
                token(Frm),
                path.clone(),
            ]),
            vec![Dependency { path }],
        );
    }

    #[test]
    fn test_namespace_export() {
        let token = new_token_factory();
        let path = token(Str("bar".to_string()));
        assert_eq!(
            parse(vec![
                token(Export),
                token(Star),
                token(As),
                token(Identifier),
                token(Frm),
                path.clone(),
                token(Eof),
            ],),
            vec![Dependency { path }],
        )
    }
}
