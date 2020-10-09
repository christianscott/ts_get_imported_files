import { Token, TokenKind } from "./token";

export type Dependency = { path: Token };

export function parse(tokens: readonly Token[]): readonly Dependency[] {
  return new Parser(tokens).parse();
}

class Parser {
  private current: number = 0;

  constructor(private readonly tokens: readonly Token[]) {}

  parse(): readonly Dependency[] {
    const deps: Dependency[] = [];
    while (!this.isAtEnd()) {
      try {
        const dep = this.importOrExport();
        dep && deps.push(dep);
      } catch (error) {
        // ...
      }
    }
    return deps;
  }

  private importOrExport(): Dependency | undefined {
    if (this.didEat(TokenKind.Import)) {
      return this.import();
    }
    return this.export();
  }

  private import(): Dependency | undefined {
    if (this.didEat(TokenKind.LeftParen)) {
      return this.dynamicImport();
    }
    return this.defaultImport();
  }

  private dynamicImport(): Dependency | undefined {
    const path = this.expect(TokenKind.String);
    this.expect(TokenKind.RightParen);
    return { path };
  }

  private defaultImport(): Dependency | undefined {
    if (this.didEat(TokenKind.Identifier)) {
      this.expect(TokenKind.From);
      const path = this.expect(TokenKind.String);
      return { path };
    }
    return this.destructuredImport();
  }

  private destructuredImport(): Dependency | undefined {
    if (this.didEat(TokenKind.LeftBrace)) {
      while (!this.nextIs(TokenKind.RightBrace) && !this.isAtEnd()) {
        this.advance();
      }

      this.expect(TokenKind.RightBrace);
      this.expect(TokenKind.From);
      return { path: this.expect(TokenKind.String) };
    }
    return this.namespaceImport();
  }

  private namespaceImport(): Dependency | undefined {
    if (this.didEat(TokenKind.Star)) {
      this.expect(TokenKind.As);
      this.expect(TokenKind.Identifier);
      this.expect(TokenKind.From);
      return { path: this.expect(TokenKind.String) };
    }

    throw new Error(
      "expected dynamic, default, destructured, or namespace import after `import` keyword"
    );
  }

  private export(): Dependency | undefined {
    if (this.didEat(TokenKind.Export)) {
      return this.destructuredExport();
    }

    this.advance();
  }

  private destructuredExport(): Dependency | undefined {
    if (this.didEat(TokenKind.LeftBrace)) {
      while (!this.nextIs(TokenKind.RightBrace) && !this.isAtEnd()) {
        this.advance();
      }

      this.expect(TokenKind.RightBrace);
      this.expect(TokenKind.From);
      return { path: this.expect(TokenKind.String) };
    }
    return this.namespaceExport();
  }

  private namespaceExport(): Dependency {
    if (this.didEat(TokenKind.Star)) {
      this.expect(TokenKind.As);
      this.expect(TokenKind.Identifier);
      this.expect(TokenKind.From);
      return { path: this.expect(TokenKind.String) };
    }

    this.advance();
  }

  private expect(kind: TokenKind) {
    if (!this.nextIs(kind)) {
      throw new Error(`expected next token to be of kind ${kind}`);
    }
    this.advance();
    return this.previous();
  }

  private didEat(kind: TokenKind): boolean {
    if (this.nextIs(kind)) {
      this.advance();
      return true;
    }
    return false;
  }

  private nextIs(kind: TokenKind) {
    return this.peek().kind === kind;
  }

  private advance(): Token {
    if (!this.isAtEnd()) {
      this.current += 1;
    }
    return this.previous();
  }

  private isAtEnd(): boolean {
    if (this.current >= this.tokens.length) {
      return true;
    }

    return this.peek().kind === TokenKind.Eof;
  }

  private peek(): Token {
    return this.peekNth(0);
  }

  private previous(): Token {
    return this.peekNth(-1);
  }

  private peekNth(n: number): Token {
    return this.tokens[this.current + n];
  }
}
