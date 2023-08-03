use std::ops::{Range, RangeFrom, RangeTo};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, multispace1, not_line_ending, space1},
    combinator::{eof, value},
    error::ParseError,
    multi::many0,
    sequence::{preceded, terminated},
    AsChar, Compare, IResult, InputIter, InputLength, InputTake, InputTakeAtPosition, Slice,
};

use crate::ConfigInInput;

pub fn ws_comment<I, E: ParseError<I>>(input: I) -> IResult<I, (), E>
where
    I: Clone + InputLength + InputTake,
    I: InputTakeAtPosition,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
    <I as InputIter>::Item: Clone,
    I: Slice<Range<usize>> + Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
    I: InputIter + InputLength,
    I: Compare<&'static str>,
    <I as InputIter>::Item: AsChar,
    <I as InputIter>::Item: AsChar,
{
    value(
        (),
        many0(alt((
            preceded(
                alt((tag("#"), tag("\\#"))),
                terminated(not_line_ending, alt((line_ending, eof))),
            ),
            preceded(
                tag(":"),
                terminated(not_line_ending, alt((line_ending, eof))),
            ),
            preceded(tag(r#"\"#), line_ending),
            preceded(
                tag("*"),
                terminated(not_line_ending, alt((line_ending, eof))),
            ),
            multispace1,
            // TODO linux v3.2, in file /drivers/dma/Kconfig
            tag(" "),
        ))),
    )(input)
}

pub fn ws<I, F, O, E: ParseError<I>>(inner: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    I: Clone + InputLength + InputTake,
    <I as InputIter>::Item: Clone,
    I: InputTakeAtPosition,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
    I: InputTakeAtPosition,
    <I as InputTakeAtPosition>::Item: AsChar,
    I: Slice<Range<usize>> + Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
    I: InputIter + InputLength,
    I: Compare<&'static str>,
    <I as InputIter>::Item: AsChar,
    <I as InputIter>::Item: AsChar,
    F: FnMut(I) -> IResult<I, O, E>,
{
    preceded(ws_comment, inner)
}

pub fn parse_until_eol(input: ConfigInInput) -> IResult<ConfigInInput, ConfigInInput> {
    terminated(not_line_ending, alt((line_ending, eof)))(input)
}

/// Gets rid of spaces, tabs and backslash + newline.
/// `wsi` for *whitespaces inline*
/// # Example
/// ```
/// use nom::bytes::complete::tag;
/// use nom_config_in::util::wsi;
/// let input = r#"   \
/// hello"#;
/// assert_eq!(wsi(tag::<&str, &str, ()>("hello"))(input), Ok(("", "hello")))
/// ```
pub fn wsi<I, F, O, E: ParseError<I>>(inner: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    I: Clone + InputLength + InputTake,
    <I as InputIter>::Item: Clone,
    I: InputTakeAtPosition,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
    I: InputTakeAtPosition,
    <I as InputTakeAtPosition>::Item: AsChar,
    I: Slice<Range<usize>> + Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
    I: InputIter + InputLength,
    I: Compare<&'static str>,
    <I as InputIter>::Item: AsChar,
    <I as InputIter>::Item: AsChar,
    F: FnMut(I) -> IResult<I, O, E>,
{
    preceded(
        value((), many0(alt((preceded(tag("\\"), line_ending), space1)))),
        inner,
    )
}
