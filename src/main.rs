mod dependency;
mod lex;
mod parse;
mod token;

fn main() {
    let deps = find_dependencies("");
    println!("{:?}", deps);
}

fn find_dependencies(source: &str) -> Vec<dependency::Dependency> {
    parse::parse(lex::lex("my module".to_string(), source))
}

#[cfg(test)]
mod find_dependencies {
    use super::*;
    use token::TokenKind;

    fn deps(source: &str) -> Vec<TokenKind> {
        find_dependencies(source)
            .iter()
            .map(|dep| dep.path.kind.clone())
            .collect()
    }

    #[test]
    fn test_dynamic_import() {
        assert_eq!(
            deps("import('bar')"),
            vec![TokenKind::Str("bar".to_string())]
        );
    }

    #[test]
    fn test_dynamic_import_with_assignment() {
        assert_eq!(
            deps("const foo = import('bar');"),
            vec![TokenKind::Str("bar".to_string())],
        );
    }

    #[test]
    fn test_dynamic_import_with_await() {
        assert_eq!(
            deps("await import('bar');"),
            vec![TokenKind::Str("bar".to_string())],
        );
    }

    #[test]
    fn test_default_import() {
        assert_eq!(
            deps("import foo from 'bar'"),
            vec![TokenKind::Str("bar".to_string())]
        );
    }

    #[test]
    fn test_multiple_imports() {
        assert_eq!(
            deps("import first from 'first';\nimport { second } from 'second'; import * as third from 'third'"),
            vec![
                TokenKind::Str("first".to_string()),
                TokenKind::Str("second".to_string()),
                TokenKind::Str("third".to_string()),
            ]
        );
    }

    #[test]
    fn test_destructured_import() {
        assert_eq!(
            deps("import {foo} from 'bar'"),
            vec![TokenKind::Str("bar".to_string())]
        );
    }

    #[test]
    fn test_namespace_import() {
        assert_eq!(
            deps("import * as foo from 'bar'"),
            vec![TokenKind::Str("bar".to_string())]
        );
    }

    #[test]
    fn test_destructured_export() {
        assert_eq!(
            deps("export {foo} from 'bar'"),
            vec![TokenKind::Str("bar".to_string())]
        );
    }

    #[test]
    fn test_namespace_export() {
        assert_eq!(
            deps("export * as foo from 'bar'"),
            vec![TokenKind::Str("bar".to_string())]
        );
    }
}
