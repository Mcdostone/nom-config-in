#[macro_export]
macro_rules! assert_parsing_eq {
    ($fn:ident, $input:expr, $expected:expr) => {{
        let res = $fn($input).map(|r| (r.0, r.1));
        assert_eq!(res, $expected)
    }};
}
