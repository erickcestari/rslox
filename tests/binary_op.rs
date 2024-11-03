use rslox::expression::Expr;
use rslox::literal::Literal;
use rslox::parser::Parser;
use rslox::statement::Stmt;
use rslox::{token::Token, token_kind::TokenKind};

#[test]
fn test_print_statement_parsing() {
    // Sample tokens for the expression: print 10 + 10 * 10;
    let tokens = vec![
        Token {
            kind: TokenKind::Print,
            lexeme: "print".to_string(),
            line: 1,
            literal: None,
        },
        Token {
            kind: TokenKind::Number,
            lexeme: "10".to_string(),
            line: 1,
            literal: Some(Literal::Number(10.0)),
        },
        Token {
            kind: TokenKind::Plus,
            lexeme: "+".to_string(),
            line: 1,
            literal: None,
        },
        Token {
            kind: TokenKind::Number,
            lexeme: "10".to_string(),
            line: 1,
            literal: Some(Literal::Number(10.0)),
        },
        Token {
            kind: TokenKind::Star,
            lexeme: "*".to_string(),
            line: 1,
            literal: None,
        },
        Token {
            kind: TokenKind::Number,
            lexeme: "10".to_string(),
            line: 1,
            literal: Some(Literal::Number(10.0)),
        },
        Token {
            kind: TokenKind::Semicolon,
            lexeme: ";".to_string(),
            line: 1,
            literal: None,
        },
        Token {
            kind: TokenKind::Eof,
            lexeme: "".to_string(),
            line: 1,
            literal: None,
        },
    ];

    let mut parser = Parser::new(tokens);
    let statements = parser.parse();

    assert_eq!(statements.len(), 1);

    if let Stmt::Print(expr) = &statements[0] {
        if let Expr::Binary(left, operator, right) = expr {
            assert!(matches!(**left, Expr::Literal(Literal::Number(10.0))));
            assert_eq!(operator.kind, TokenKind::Plus);
            if let Expr::Binary(left_mult, operator_mult, right_mult) = &**right {
                assert!(matches!(**left_mult, Expr::Literal(Literal::Number(10.0))));
                assert_eq!(operator_mult.kind, TokenKind::Star);
                assert!(matches!(**right_mult, Expr::Literal(Literal::Number(10.0))));
            } else {
                panic!("Expected a binary expression for the right operand of '+'");
            }
        } else {
            panic!("Expected a binary expression for the print statement");
        }
    } else {
        panic!("Expected a print statement");
    }
}
