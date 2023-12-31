use crate::{
    assert_parsing_eq,
    entry::expression::{
        parse_expression, AndExpression, Atom, CompareExpression, CompareOperator, Expression, Term,
    },
};

#[test]
fn test_parse_expression_number() {
    assert_parsing_eq!(
        parse_expression,
        "-412",
        Ok((
            "",
            Expression::Term(AndExpression::Term(Term::Atom(Atom::Number(-412))))
        ))
    )
}

#[test]
fn test_parse_term() {
    assert_parsing_eq!(
        parse_expression,
        "!KVM",
        Ok((
            "",
            Expression::Term(AndExpression::Term(Term::Not(Atom::Symbol(
                "KVM".to_string()
            ))))
        ))
    )
}

#[test]
fn test_parse_depends_on_and() {
    assert_parsing_eq!(
        parse_expression,
        "ALPHA_MIATA -a ALPHA_LX164",
        Ok((
            "",
            Expression::Term(AndExpression::Expression(vec!(
                Term::Atom(Atom::Symbol("ALPHA_MIATA".to_string())),
                Term::Atom(Atom::Symbol("ALPHA_LX164".to_string())),
            )))
        ))
    )
}

#[test]
fn test_parse_depends_on_ambigus() {
    assert_parsing_eq!(
        parse_expression,
        "ALPHA_MIATA -o ALPHA_LX164 -a ALPHA_SX164",
        Ok((
            "",
            Expression::Expression(vec!(
                AndExpression::Term(Term::Atom(Atom::Symbol("ALPHA_MIATA".to_string()))),
                AndExpression::Expression(vec!(
                    Term::Atom(Atom::Symbol("ALPHA_LX164".to_string())),
                    Term::Atom(Atom::Symbol("ALPHA_SX164".to_string())),
                ))
            ))
        ))
    )
}

#[test]
fn test_parse_expression_parenthesis() {
    assert_parsing_eq!(
        parse_expression,
        "(hello = world)",
        Ok((
            "",
            Expression::Term(AndExpression::Term(Term::Atom(Atom::Parenthesis(
                Box::new(Expression::Term(AndExpression::Term(Term::Atom(
                    Atom::Compare(CompareExpression {
                        left: "hello".to_string(),
                        operator: CompareOperator::Equal,
                        right: "world".to_string()
                    })
                ))))
            ))))
        ))
    )
}

#[test]
fn test_parse_depends_on_optimization() {
    assert_parsing_eq!(
        parse_expression,
        "ALPHA_MIATA -o ALPHA_LX164 -a ALPHA_SX164 -a (HELLO = world) -o ALPHA_SX164 -a (HELLO = world)",
        Ok(("", Expression::Expression(
            vec!(
                AndExpression::Term(Term::Atom(Atom::Symbol("ALPHA_MIATA".to_string()))),
                AndExpression::Expression(vec!(
                    Term::Atom(Atom::Symbol("ALPHA_LX164".to_string())),
                    Term::Atom(Atom::Symbol("ALPHA_SX164".to_string())),
                    Term::Atom(Atom::Parenthesis(Box::new(Expression::Term(AndExpression::Term(Term::Atom(Atom::Compare(
                        CompareExpression { left: "HELLO".to_string(), operator: CompareOperator::Equal, right: "world".to_string() }
                    ))))))),
                )),
                AndExpression::Expression(vec!(
                    Term::Atom(Atom::Symbol("ALPHA_SX164".to_string())),
                    Term::Atom(Atom::Parenthesis(Box::new(Expression::Term(AndExpression::Term(Term::Atom(Atom::Compare(CompareExpression { left: "HELLO".to_string(), operator: CompareOperator::Equal, right: "world".to_string()})))))))
                ))
            )
        ))))
}

#[test]
fn test_parse_expression_file_exists() {
    let input = "[ -f drivers/net/soundmodem/sm_afsk2666.c ]";
    assert_parsing_eq!(
        parse_expression,
        input,
        Ok((
            "",
            Expression::Term(AndExpression::Term(Term::Atom(Atom::Bracket(Box::new(
                Expression::Term(AndExpression::Term(Term::Atom(Atom::Existance(
                    "drivers/net/soundmodem/sm_afsk2666.c".to_string()
                ))))
            )))))
        ))
    )
}
