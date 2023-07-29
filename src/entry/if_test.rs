use crate::{
    entry::{
        def_bool::DefBool,
        expression::{
            AndExpression, Atom, CompareExpression, CompareOperator, Expression, OrExpression, Term,
        },
        r#if::{parse_if, If},
        r#type::Type,
    },
    Entry,
};

use crate::assert_parsing_eq;

#[test]
fn test_parse_if_entry() {
    let input = r#"if [ "$CONFIG_SCSI" = "n" ]; then comment 'Skipping SCSI configuration options...'
    fi"#;
    assert_parsing_eq!(
        parse_if,
        input,
        Ok((
            "",
            If {
                condition: Expression(OrExpression::Term(AndExpression::Term(Term::Atom(
                    Atom::Parenthesis(Box::new(Expression(OrExpression::Term(
                        AndExpression::Term(Term::Atom(Atom::Compare(CompareExpression {
                            left: "\"$CONFIG_SCSI\"".to_string(),
                            operator: CompareOperator::Equal,
                            right: "\"n\"".to_string()
                        })))
                    ))))
                )))),
                if_block: vec!(Entry::Comment(
                    "Skipping SCSI configuration options...".to_string()
                )),
                else_block: None
            }
        ))
    )
}

#[test]
fn test_parse_if_entry_quote_operator() {
    let input = r#"if [ "$CONFIG_SCSI" "=" "n" ]; then comment 'Skipping SCSI configuration options...'
    fi"#;
    assert_parsing_eq!(
        parse_if,
        input,
        Ok((
            "",
            If {
                condition: Expression(OrExpression::Term(AndExpression::Term(Term::Atom(
                    Atom::Parenthesis(Box::new(Expression(OrExpression::Term(
                        AndExpression::Term(Term::Atom(Atom::Compare(CompareExpression {
                            left: "\"$CONFIG_SCSI\"".to_string(),
                            operator: CompareOperator::Equal,
                            right: "\"n\"".to_string()
                        })))
                    ))))
                )))),
                if_block: vec!(Entry::Comment(
                    "Skipping SCSI configuration options...".to_string()
                )),
                else_block: None
            }
        ))
    )
}

#[test]
fn test_parse_if_else_entry() {
    let input = r#"if [ "$CONFIG_SCSI" = "n" ]; then comment 'Skipping SCSI configuration options...' else bool 'Scsi disk support' CONFIG_BLK_DEV_SD y
    fi"#;
    assert_parsing_eq!(
        parse_if,
        input,
        Ok((
            "",
            If {
                condition: Expression(OrExpression::Term(AndExpression::Term(Term::Atom(
                    Atom::Parenthesis(Box::new(Expression(OrExpression::Term(
                        AndExpression::Term(Term::Atom(Atom::Compare(CompareExpression {
                            left: "\"$CONFIG_SCSI\"".to_string(),
                            operator: CompareOperator::Equal,
                            right: "\"n\"".to_string()
                        })))
                    ))))
                )))),
                if_block: vec!(Entry::Comment(
                    "Skipping SCSI configuration options...".to_string()
                )),
                else_block: Some(vec!(Entry::Bool(Type {
                    symbol: "CONFIG_BLK_DEV_SD".to_string(),
                    prompt: "Scsi disk support".to_string(),
                    value: Some("y".to_string())
                })))
            }
        ))
    )
}

#[test]
fn test_parse_if_else_backtick() {
    let input = r#"if [ "`uname`" != "Linux" ]; then define_bool CONFIG_CROSSCOMPILE y else define_bool CONFIG_NATIVE y fi"#;
    assert_parsing_eq!(
        parse_if,
        input,
        Ok((
            "",
            If {
                condition: Expression(OrExpression::Term(AndExpression::Term(Term::Atom(
                    Atom::Parenthesis(Box::new(Expression(OrExpression::Term(
                        AndExpression::Term(Term::Atom(Atom::Compare(CompareExpression {
                            left: "\"`uname`\"".to_string(),
                            operator: CompareOperator::NotEqual,
                            right: "\"Linux\"".to_string()
                        })))
                    ))))
                )))),
                if_block: vec!(Entry::DefBool(DefBool {
                    symbol: "CONFIG_CROSSCOMPILE".to_string(),
                    values: vec!("y".to_string())
                })),
                else_block: Some(vec!(Entry::DefBool(DefBool {
                    symbol: "CONFIG_NATIVE".to_string(),
                    values: vec!("y".to_string())
                })))
            }
        ))
    )
}
