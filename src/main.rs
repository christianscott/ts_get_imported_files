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
mod test {
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
    fn test_default_import() {
        assert_eq!(
            deps("import foo from 'bar'"),
            vec![TokenKind::Str("bar".to_string())]
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
}
