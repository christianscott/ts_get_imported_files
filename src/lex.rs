use crate::token::{Range, Source, Token, TokenKind};
use std::rc::Rc;

pub fn lex(name: String, source: &str) -> Vec<Token> {
    Lexer::new(name, source.chars().collect()).lex()
}

struct Lexer {
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    source: Rc<Source>,
}

impl Lexer {
    fn new(name: String, chars: Vec<char>) -> Self {
        let source = Source::new(name, chars);
        Self {
            source: Rc::new(source),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn lex(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }
        self.add_basic_token(TokenKind::Eof);

        self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance().clone();
        match c {
            '(' => self.add_basic_token(TokenKind::LeftParen),
            ')' => self.add_basic_token(TokenKind::RightParen),
            '{' => self.add_basic_token(TokenKind::LeftBrace),
            '}' => self.add_basic_token(TokenKind::RightBrace),
            ',' => self.add_basic_token(TokenKind::Comma),
            ';' => self.add_basic_token(TokenKind::Semicolon),
            '*' => self.add_basic_token(TokenKind::Star),
            ' ' | '\t' | '\r' => {} // skip whitespace
            '\n' => {
                self.line += 1;
            }
            '"' | '\'' | '`' => self.string(c),
            _ => {
                if c.is_alphabetic() {
                    self.identifier();
                }
            }
        }
    }

    fn string(&mut self, open_quote: char) {
        self.eat_while(|&ch| ch != open_quote);
        if self.is_at_end() {
            // TODO: error out here
            return;
        }
        self.advance();
        let text = self.get_lexeme(&Range(self.start + 1, self.current - 1));
        self.add_basic_token(TokenKind::Str(text))
    }

    fn identifier(&mut self) {
        self.eat_while(|c| c.is_alphanumeric());
        let text = self.get_current_lexeme();
        let kind = token_kind_for_text(&text);
        self.add_basic_token(kind);
    }

    fn get_current_lexeme(&self) -> String {
        self.get_lexeme(&Range(self.start, self.current))
    }

    fn get_lexeme(&self, range: &Range) -> String {
        self.source.range(range).iter().clone().collect()
    }

    fn eat(&mut self, c: char) -> bool {
        if self.next_is(c) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn eat_while<Pred: Fn(&char) -> bool>(&mut self, predicate: Pred) {
        while self.peek().map_or(false, |c| predicate(c)) {
            self.advance();
        }
    }

    fn next_is(&self, c: char) -> bool {
        self.peek().map_or(false, |&ch| ch == c)
    }

    fn advance(&mut self) -> &char {
        self.current += 1;
        self.source.get_unchecked(self.current - 1)
    }

    fn peek(&self) -> Option<&char> {
        self.peek_nth(0)
    }

    fn peek_nth(&self, n: usize) -> Option<&char> {
        self.source.get(self.current + n)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_basic_token(&mut self, kind: TokenKind) {
        self.add_token(self.token(kind));
    }

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn token(&self, kind: TokenKind) -> Token {
        Token {
            source: Rc::clone(&self.source),
            kind,
            span: Range(self.start, self.current),
            line: self.line,
        }
    }
}

/// get the token kind (sans literal) for a piece of text. falls back to "identifier"
fn token_kind_for_text(text: &str) -> TokenKind {
    match text.as_ref() {
        "import" => TokenKind::Import,
        "export" => TokenKind::Export,
        "as" => TokenKind::As,
        "from" => TokenKind::Frm,
        "type" => TokenKind::Type,
        _ => TokenKind::Identifier,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::token::TokenKind::*;

    fn to_token_kinds(source: &str) -> Vec<TokenKind> {
        lex("<for testing>".to_string(), source)
            .iter()
            .map(|token| token.kind.clone())
            .collect()
    }

    #[test]
    fn test_default_import() {
        assert_eq!(
            to_token_kinds("import foo from './bar'"),
            vec![Import, Identifier, Frm, Str("./bar".to_string()), Eof],
        );
    }

    #[test]
    fn test_destructured_import() {
        assert_eq!(
            to_token_kinds("import {one, two, three} from \"module\""),
            vec![
                Import,
                LeftBrace,
                Identifier,
                Comma,
                Identifier,
                Comma,
                Identifier,
                RightBrace,
                Frm,
                Str("module".to_string()),
                Eof,
            ],
        );
    }

    #[test]
    fn test_namespace_import() {
        assert_eq!(
            to_token_kinds("import * as foo from 'bar'"),
            vec![
                Import,
                Star,
                As,
                Identifier,
                Frm,
                Str("bar".to_string()),
                Eof,
            ],
        );
    }

    #[test]
    fn test_dynamic_import() {
        assert_eq!(
            to_token_kinds("const { one, two, three } = import(`../../something`)"),
            vec![
                Identifier,
                LeftBrace,
                Identifier,
                Comma,
                Identifier,
                Comma,
                Identifier,
                RightBrace,
                Import,
                LeftParen,
                Str("../../something".to_string()),
                RightParen,
                Eof,
            ],
        );
    }

    #[test]
    fn test_destructured_export() {
        assert_eq!(
            to_token_kinds("export { one, two } from 'bar'"),
            vec![
                Export,
                LeftBrace,
                Identifier,
                Comma,
                Identifier,
                RightBrace,
                Frm,
                Str("bar".to_string()),
                Eof,
            ],
        );
    }

    #[test]
    fn test_namespace_export() {
        assert_eq!(
            to_token_kinds("export * as bar from 'bar'"),
            vec![
                Export,
                Star,
                As,
                Identifier,
                Frm,
                Str("bar".to_string()),
                Eof,
            ],
        );
    }
}
