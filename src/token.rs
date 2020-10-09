use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Source {
    name: String,
    chars: Vec<char>,
}

impl Source {
    pub fn new(name: String, chars: Vec<char>) -> Self {
        Self { name, chars }
    }

    pub fn range(&self, range: &Range) -> &[char] {
        &self.chars[range.0..range.1]
    }

    pub fn get_unchecked(&self, index: usize) -> &char {
        unsafe { self.chars.get_unchecked(index) }
    }

    pub fn get(&self, index: usize) -> Option<&char> {
        self.chars.get(index)
    }

    pub fn len(&self) -> usize {
        self.chars.len()
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Range(pub usize, pub usize);

#[derive(Clone, Debug, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Range,
    pub line: usize,
    pub source: Rc<Source>,
}

impl Token {
    pub fn name(&self) -> String {
        self.source.range(&self.span).iter().collect()
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        self.kind == other.kind
            && self.span == other.span
            && self.line == other.line
            && self.source.name == other.source.name
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TokenKind {
    // SingleCharacterTokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,
    Star,

    // Literals
    Identifier,
    Str(String),

    // Keywords
    Export,
    Import,
    Frm,
    As,
    Type,
    Eof,
}
