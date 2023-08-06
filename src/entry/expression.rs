use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map, map_res, opt, recognize, value},
    multi::many0,
    sequence::{delimited, pair, preceded, tuple, terminated},
    IResult,
};
use serde::Serialize;

use crate::{
    symbol::{parse_symbol, Symbol},
    util::{ws, wsi},
    ConfigInInput,
};

use super::source::parse_path;

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
    DoubleEqual,
    NotEqual,
}

// https://stackoverflow.com/questions/9509048/antlr-parser-for-and-or-logic-how-to-get-expressions-between-logic-operators
pub type Expression = OrExpression;
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
    Symbol(String),
    Number(i64),
    Existance(String),
    Compare(CompareExpression),
    Parenthesis(Box<Expression>),
    Bracket(Box<Expression>),
    String(Box<Atom>),
}

pub fn parse_or_expression(input: ConfigInInput) -> IResult<ConfigInInput, Expression> {
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

pub fn parse_and_expression(input: ConfigInInput) -> IResult<ConfigInInput, AndExpression> {
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

pub fn parse_term(input: ConfigInInput) -> IResult<ConfigInInput, Term> {
    alt((
        map(
            preceded(ws(alt((tag("-n"), tag("!")))), parse_atom),
            Term::Not,
        ),
        map(parse_atom, Term::Atom),
    ))(input)
}

pub fn parse_atom(input: ConfigInInput) -> IResult<ConfigInInput, Atom> {
    alt((
        map(
            delimited(ws(tag("(")), parse_expression, ws(tag(")"))),
            |expr| Atom::Parenthesis(Box::new(expr)),
        ),
        map(
            delimited(ws(tag("[")), parse_expression, ws(tag("]"))),
            |expr| Atom::Bracket(Box::new(expr)),
        ),
        ws(parse_compare),
        map(delimited(tag("\""), parse_atom, tag("\"")), |d| {
            Atom::String(Box::new(d))
        }),
        map(parse_entry_exists, Atom::Existance),
        map(parse_number, Atom::Number),
        map(parse_symbol, Atom::Symbol),
    ))(input)
}

pub fn parse_entry_exists(input: ConfigInInput) -> IResult<ConfigInInput, String> {
    //let t = tag("-f")(input);
    map(preceded(wsi(tag("-f")), wsi(parse_path)), |f| f.to_string())(input)
}



pub fn parse_expression(input: ConfigInInput) -> IResult<ConfigInInput, Expression> {
    parse_or_expression(input)
}

pub fn parse_compare_operator(input: ConfigInInput) -> IResult<ConfigInInput, CompareOperator> {
    alt((
        value(CompareOperator::GreaterOrEqual, tag(">=")),
        value(CompareOperator::LowerOrEqual, tag("<=")),
        value(CompareOperator::GreaterThan, tag(">")),
        value(CompareOperator::LowerThan, tag("<")),
        value(CompareOperator::DoubleEqual, tag("==")),
        value(CompareOperator::Equal, tag("=")),
        value(CompareOperator::NotEqual, tag("!=")),
    ))(input)
}

pub fn parse_compare(input: ConfigInInput) -> IResult<ConfigInInput, Atom> {
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

pub fn parse_number(input: ConfigInInput) -> IResult<ConfigInInput, i64> {
    map_res(
        recognize(pair(opt(char('-')), digit1)),
        |d: ConfigInInput| FromStr::from_str(d.fragment()),
    )(input)
}
