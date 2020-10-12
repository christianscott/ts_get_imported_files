import { Char, Source, Token, TokenKind } from "./token";

export function lex(source: Source): readonly Token[] {
  return new Lexer(source).lex();
}

class Lexer {
  private readonly tokens: Token[] = [];
  private start: number = 0;
  private current: number = 0;
  private line: number = 1;

  constructor(private readonly source: Source) {}

  lex(): readonly Token[] {
    while (!this.isAtEnd()) {
      this.start = this.current;
      this.scanToken();
    }
    return this.tokens;
  }

  private scanToken(): void {
    const c = this.advance();
    switch (c) {
      case " ":
      case "\t":
      case "\r":
        return; // skip whitespace
      case "(":
        this.addBasicToken(TokenKind.LeftParen);
        return;
      case ")":
        this.addBasicToken(TokenKind.RightParen);
        return;
      case "{":
        this.addBasicToken(TokenKind.LeftBrace);
        return;
      case "}":
        this.addBasicToken(TokenKind.RightBrace);
        return;
      case ",":
        this.addBasicToken(TokenKind.Comma);
        return;
      case ";":
        this.addBasicToken(TokenKind.Semicolon);
        return;
      case "*":
        this.addBasicToken(TokenKind.Star);
        return;
      case "\n":
        this.line++;
        return;
      case '"':
      case "'":
      case "`":
        this.string(c);
        return;
      default: {
        if (isAlphabetic(c)) {
          this.identifier();
        }
        return;
      }
    }
  }

  private string(openQuote: Char): void {
    this.eatWhile((c) => c !== openQuote);
    this.advance();
    const text = this.getLexeme([this.start + 1, this.current - 1]);
    this.addBasicToken(TokenKind.String, text);
  }

  private identifier(): void {
    this.eatWhile((c) => isAlphaNumeric(c));
    const text = this.getCurrentLexeme();
    const kind = tokenKindForText(text);
    this.addBasicToken(kind);
  }

  private getCurrentLexeme(): string {
    return this.getLexeme([this.start, this.current]);
  }

  private getLexeme(range: [number, number]): string {
    return this.source.charsWithin(range);
  }

  private addBasicToken(kind: TokenKind, lexeme?: string): void {
    this.addToken(this.token(kind, lexeme));
  }

  private addToken(token: Token): void {
    this.tokens.push(token);
  }

  private eatWhile(predicate: (c: Char) => boolean): void {
    while (!this.isAtEnd() && predicate(this.peek())) {
      this.advance();
    }
  }

  private peek(): Char {
    return this.peekNth(0);
  }

  private peekNth(n: number): Char {
    return this.source.charAt(this.current + n);
  }

  private advance(): Char {
    this.current++;
    return this.source.charAt(this.current - 1);
  }

  private isAtEnd(): boolean {
    return this.current >= this.source.length();
  }

  private token(kind: TokenKind, lexeme?: string): Token {
    return {
      kind,
      lexeme,
      line: this.line,
      source: this.source,
      span: [this.start, this.current],
    };
  }
}

function tokenKindForText(text: string): TokenKind {
  switch (text) {
    case "import":
      return TokenKind.Import;
    case "export":
      return TokenKind.Export;
    case "as":
      return TokenKind.As;
    case "from":
      return TokenKind.From;
    case "type":
      return TokenKind.Type;
    default:
      return TokenKind.Identifier;
  }
}

function isAlphabetic(c: Char): boolean {
  return /[a-zA-Z]/.test(c);
}

function isAlphaNumeric(c: Char) {
  return /[a-zA-Z0-9]/.test(c);
}
