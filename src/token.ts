import { Preconditions } from "./preconditions";

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

export type Char = string & { __char: never };

export class Source {
  private readonly chars: readonly Char[];

  constructor(readonly name: string, source: string) {
    this.chars = [...source] as Char[];
  }

  charsWithin([start, end]: [number, number]): string {
    return this.chars.slice(start, end).join("");
  }

  charAt(index: number): Char {
    return Preconditions.checkExists(this.chars[index]);
  }

  length() {
    return this.chars.length;
  }
}
