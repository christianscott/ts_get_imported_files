export enum TokenKind {
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
  String,

  // Keywords
  Export,
  Import,
  From,
  As,
  Type,
  Eof,
}

export type Token = {
  kind: TokenKind;
  span: [number, number];
  line: number;
  source: Source;
  lexeme?: string;
};

export class Source {
  readonly chars: readonly Char[];
  constructor(readonly name: string, source: string) {
    this.chars = [...source] as Char[];
  }
}

export type Char = string & { __char: never };
