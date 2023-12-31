use crate::entry::{
    define_type::DefineBool,
    expression::{AndExpression, Atom, CompareExpression, CompareOperator, Expression, Term},
    r#if::{parse_if, If},
    r#type::{Type, TypeEnum},
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
                condition: Expression::Term(AndExpression::Term(Term::Atom(Atom::Bracket(
                    Box::new(Expression::Term(AndExpression::Term(Term::Atom(
                        Atom::Compare(CompareExpression {
                            left: "\"$CONFIG_SCSI\"".to_string(),
                            operator: CompareOperator::Equal,
                            right: "\"n\"".to_string()
                        })
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
                condition: Expression::Term(AndExpression::Term(Term::Atom(Atom::Bracket(
                    Box::new(Expression::Term(AndExpression::Term(Term::Atom(
                        Atom::Compare(CompareExpression {
                            left: "\"$CONFIG_SCSI\"".to_string(),
                            operator: CompareOperator::Equal,
                            right: "\"n\"".to_string()
                        })
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
                condition: Expression::Term(AndExpression::Term(Term::Atom(Atom::Bracket(
                    Box::new(Expression::Term(AndExpression::Term(Term::Atom(
                        Atom::Compare(CompareExpression {
                            left: "\"$CONFIG_SCSI\"".to_string(),
                            operator: CompareOperator::Equal,
                            right: "\"n\"".to_string()
                        })
                    ))))
                )))),
                if_block: vec!(Entry::Comment(
                    "Skipping SCSI configuration options...".to_string()
                )),
                else_block: Some(vec!(Entry::Bool(Type {
                    symbol: "CONFIG_BLK_DEV_SD".to_string(),
                    r#type: crate::entry::r#type::TypeEnum::Bool,
                    prompt: "Scsi disk support".to_string(),
                    value: vec!("y".to_string())
                })))
            }
        ))
    )
}

#[test]
fn test_parse_if_else_backtick() {
    let input = r#"if [ "`uname`" != "Linux" ]; then define_bool CONFIG_CROSSCOMPILE y 
    else 
    define_bool CONFIG_NATIVE y 
    fi"#;
    assert_parsing_eq!(
        parse_if,
        input,
        Ok((
            "",
            If {
                condition: Expression::Term(AndExpression::Term(Term::Atom(Atom::Bracket(
                    Box::new(Expression::Term(AndExpression::Term(Term::Atom(
                        Atom::Compare(CompareExpression {
                            left: "\"`uname`\"".to_string(),
                            operator: CompareOperator::NotEqual,
                            right: "\"Linux\"".to_string()
                        })
                    ))))
                )))),
                if_block: vec!(Entry::DefineBool(DefineBool {
                    symbol: "CONFIG_CROSSCOMPILE".to_string(),
                    value: vec!("y".to_string()),
                    r#type: TypeEnum::Bool,
                })),
                else_block: Some(vec!(Entry::DefineBool(DefineBool {
                    symbol: "CONFIG_NATIVE".to_string(),
                    value: vec!("y".to_string()),
                    r#type: TypeEnum::Bool,
                })),)
            }
        ))
    )
}

// 2.1.43/drivers/net/Config.in
#[test]
fn test_parse_if_file_exists() {
    let input = r#"if [ -f drivers/net/soundmodem/sm_afsk2666.c ]; then
    bool 'Soundmodem support for 2666 baud AFSK modulation' CONFIG_SOUNDMODEM_AFSK2666
    fi"#;
    assert_parsing_eq!(
        parse_if,
        input,
        Ok((
            "",
            If {
                condition: Expression::Term(AndExpression::Term(Term::Atom(Atom::Bracket(
                    Box::new(Expression::Term(AndExpression::Term(Term::Atom(
                        Atom::Existance("drivers/net/soundmodem/sm_afsk2666.c".to_string())
                    ))))
                )))),
                if_block: vec!(Entry::Bool(Type {
                    prompt: "Soundmodem support for 2666 baud AFSK modulation".to_string(),
                    symbol: "CONFIG_SOUNDMODEM_AFSK2666".to_string(),
                    value: vec!(),
                    r#type: TypeEnum::Bool,
                })),
                else_block: None
            }
        ))
    )
}

// 2.4.25/arch/cris/drivers/Config.in
#[test]
fn test_parse_if_variant_2() {
    let input = r#"if [ "$CONFIG_MTD_CFI" = "n" ] && [ "$CONFIG_MTD_AMDSTD" = "n" ] && \
    [ "$CONFIG_MTD_MTDRAM" = "n" ]; then
    fi"#;
    assert_parsing_eq!(
        parse_if,
        input,
        Ok((
            "",
            If {
                condition: Expression::Term(AndExpression::Expression(vec!(
                    Term::Atom(Atom::Bracket(Box::new(Expression::Term(
                        AndExpression::Term(Term::Atom(Atom::Compare(CompareExpression {
                            left: "\"$CONFIG_MTD_CFI\"".to_string(),
                            operator: CompareOperator::Equal,
                            right: "\"n\"".to_string()
                        })))
                    )))),
                    Term::Atom(Atom::Bracket(Box::new(Expression::Term(
                        AndExpression::Term(Term::Atom(Atom::Compare(CompareExpression {
                            left: "\"$CONFIG_MTD_AMDSTD\"".to_string(),
                            operator: CompareOperator::Equal,
                            right: "\"n\"".to_string()
                        })))
                    )))),
                    Term::Atom(Atom::Bracket(Box::new(Expression::Term(
                        AndExpression::Term(Term::Atom(Atom::Compare(CompareExpression {
                            left: "\"$CONFIG_MTD_MTDRAM\"".to_string(),
                            operator: CompareOperator::Equal,
                            right: "\"n\"".to_string()
                        })))
                    ))))
                ))),
                if_block: vec!(),
                else_block: None
            }
        ))
    )
}
