use crate::{
    symbol::Symbol,
    Entry, entry::{expression::{OrExpression, CompareExpression, Expression, AndExpression, Term, CompareOperator, Atom}, comment::Comment, r#if::{parse_if, If}, bool::Bool}, 
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
                    Atom::Compare(CompareExpression { 
                        left: Symbol::NonConstant("\"$CONFIG_SCSI\"".to_string()), 
                        operator: CompareOperator::Equal, 
                        right: Symbol::NonConstant("\"n\"".to_string())  })
                )))),
                if_block: vec!( Entry::Comment(Comment {
                    prompt: "Skipping SCSI configuration options...".to_string()
                })),
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
                    Atom::Compare(CompareExpression { 
                        left: Symbol::NonConstant("\"$CONFIG_SCSI\"".to_string()), 
                        operator: CompareOperator::Equal, 
                        right: Symbol::NonConstant("\"n\"".to_string())  })
                )))),
                if_block: vec!(Entry::Comment(Comment {
                    prompt: "Skipping SCSI configuration options...".to_string()
                })),
                else_block: Some(vec!(
                    Entry::Bool(Bool {
                        symbol: Symbol::Constant("CONFIG_BLK_DEV_SD".to_string()),
                        prompt: "Scsi disk support".to_string(),
                        default: Some("y".to_string())
                    })
                ))
            }
        ))
    )
}