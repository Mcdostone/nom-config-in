#[macro_export]
macro_rules! assert_parsing_eq {
    ($fn:ident, $input:expr, $expected:expr) => {{
        use $crate::ConfigInInput;
        let res = $fn(ConfigInInput::new_extra($input, Default::default()))
            .map(|r| (r.0.fragment().to_owned(), r.1))
            .map_err(|e| e.to_string());
        assert_eq!(res, $expected)
    }};
}

#[macro_export]
macro_rules! assert_parsing_fail {
    ($fn:ident, $input:expr) => {{
        use $crate::KconfigInput;
        let res = $fn(KconfigInput::new_extra($input, Default::default()));
        assert!(res.is_err())
    }};
}

#[macro_export]
macro_rules! assert_parsing_source_eq {
    ($fn:ident, $input:expr, $expected:expr) => {{
        use $crate::KconfigInput;
        let res = $fn(KconfigInput::new_extra(
            $input,
            KconfigFile {
                ..Default::default()
            },
        ))
        .map(|r| (r.0.fragment().to_owned(), r.1));
        assert_eq!(res, $expected)
    }};
}
