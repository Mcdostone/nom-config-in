use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, space1},
    combinator::{map, map_res, opt, recognize, value},
    multi::many0,
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};
use serde::Serialize;

use crate::{
    symbol::{parse_symbol, Symbol},
    util::ws,
};

// (GFS2_FS!=n) && NET && INET && (IPV6 || IPV6=n) && CONFIGFS_FS && SYSFS && (DLM=y || DLM=GFS2_FS)

#[derive(Debug, Serialize, PartialEq, Clone)]
pub enum Operator {
    And,
    Or,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum CompareOperator {
    GreaterThan,
    GreaterOrEqual,
    LowerThan,
    LowerOrEqual,
    Equal,
    NotEqual,
}

// https://stackoverflow.com/questions/9509048/antlr-parser-for-and-or-logic-how-to-get-expressions-between-logic-operators

#[derive(Debug, Serialize, PartialEq, Clone, Default)]
pub struct Expression(pub OrExpression);
#[derive(Debug, Serialize, PartialEq, Clone)]
pub enum AndExpression {
    #[serde(rename = "AndTerm")]
    Term(Term),

    #[serde(rename = "And")]
    Expression(Vec<Term>),
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub enum OrExpression {
    #[serde(rename = "OrTerm")]
    Term(AndExpression),
    #[serde(rename = "Or")]
    Expression(Vec<AndExpression>),
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub enum Term {
    Not(Atom),
    Atom(Atom),
}

#[derive(Debug, Serialize, PartialEq, Clone)]
#[serde(rename = "Compare")]
pub struct CompareExpression {
    pub left: Symbol,
    pub operator: CompareOperator,
    pub right: Symbol,
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub enum Atom {
    Symbol(Symbol),
    Number(i64),
    Compare(CompareExpression),
    Parenthesis(Box<Expression>),
    String(Box<Atom>),
}

impl Default for OrExpression {
    fn default() -> Self {
        Self::Term(Default::default())
    }
}

impl Default for AndExpression {
    fn default() -> Self {
        Self::Term(Default::default())
    }
}

impl Default for Term {
    fn default() -> Self {
        Self::Atom(Default::default())
    }
}
impl Default for Atom {
    fn default() -> Self {
        Self::Symbol(Default::default())
    }
}

pub fn parse_or_expression(input: &str) -> IResult<&str, OrExpression> {
    map(
        tuple((
            ws(parse_and_expression),
            many0(preceded(
                alt((ws(tag("-o")), ws(tag("||")))),
                ws(parse_and_expression),
            )),
        )),
        |(l, ee)| {
            if ee.is_empty() {
                OrExpression::Term(l)
            } else {
                let mut ll = vec![l];
                ll.extend(ee);
                OrExpression::Expression(ll)
            }
        },
    )(input)
}

pub fn parse_and_expression(input: &str) -> IResult<&str, AndExpression> {
    map(
        tuple((
            ws(parse_term),
            many0(preceded(ws(tag("-a")), ws(parse_term))),
        )),
        |(l, ee)| {
            if ee.is_empty() {
                AndExpression::Term(l)
            } else {
                let mut ll = vec![l];
                ll.extend(ee);
                AndExpression::Expression(ll)
            }
        },
    )(input)
}

pub fn parse_term(input: &str) -> IResult<&str, Term> {
    alt((
        map(
            preceded(ws(alt((tag("-n"), tag("!")))), parse_atom),
            Term::Not,
        ),
        map(parse_atom, Term::Atom),
    ))(input)
}

pub fn parse_atom(input: &str) -> IResult<&str, Atom> {
    alt((
        ws(parse_compare),
        map(delimited(tag("\""), parse_atom, tag("\"")), |d| {
            Atom::String(Box::new(d))
        }),
        map(parse_number, Atom::Number),
        map(parse_symbol, Atom::Symbol),
        map(
            delimited(ws(tag("[")), parse_expression, ws(tag("]"))),
            |expr| Atom::Parenthesis(Box::new(expr)),
        ),
        map(parse_symbol, Atom::Symbol),
    ))(input)
}

pub fn parse_expression(input: &str) -> IResult<&str, Expression> {
    map(parse_or_expression, Expression)(input)
}

pub fn parse_compare_operator(input: &str) -> IResult<&str, CompareOperator> {
    alt((
        value(CompareOperator::GreaterOrEqual, tag(">=")),
        value(CompareOperator::LowerOrEqual, tag("<=")),
        value(CompareOperator::GreaterThan, tag(">")),
        value(CompareOperator::LowerThan, tag("<")),
        value(CompareOperator::Equal, tag("==")),
        value(CompareOperator::Equal, tag("=")),
        value(CompareOperator::NotEqual, tag("!=")),
    ))(input)
}

pub fn parse_compare(input: &str) -> IResult<&str, Atom> {
    map(
        tuple((
            ws(parse_symbol),
            alt((
                ws(delimited(tag("\""), ws(parse_compare_operator), tag("\""))),
                ws(parse_compare_operator),
            )),
            ws(parse_symbol),
        )),
        |(l, o, r)| {
            Atom::Compare(CompareExpression {
                left: l,
                operator: o,
                right: r,
            })
        },
    )(input)
}

pub fn parse_if_expression_attribute(input: &str) -> IResult<&str, Expression> {
    map(
        tuple((space1, tag("if"), ws(parse_expression))),
        |(_, _, e)| e,
    )(input)
}

pub fn parse_hex_number(input: &str) -> IResult<&str, i64> {
    map_res(recognize(pair(opt(char('-')), digit1)), FromStr::from_str)(input)
}

pub fn parse_number(input: &str) -> IResult<&str, i64> {
    map_res(recognize(pair(opt(char('-')), digit1)), FromStr::from_str)(input)
}
