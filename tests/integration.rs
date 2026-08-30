use with::{WithOpt, withopt};

#[test]
fn test_config_parse() {
    #[derive(withopt)]
    struct Config {
        path: String,
        second_moment: f32,
        verbose: bool,
    }

    assert!(
        Config::parse(&[
            "myprogramname",
            "with",
            "path",
            "as",
            "./example.rs",
            "with",
            "second",
            "moment",
            "as",
            "4.1192",
            "with",
            "verbose",
            "as",
            "false"
        ])
        .is_ok()
    )
}

#[test]
fn test_empty_struct() {
    #[derive(withopt, Debug)]
    struct Config {}

    let superfluous = &[
        "myprogramname",
        "with",
        "path",
        "as",
        "./example.rs",
        "with",
        "second",
        "moment",
        "as",
        "4.1192",
        "with",
        "verbose",
        "as",
        "false",
    ];

    assert!(Config::parse(superfluous).is_ok());
    assert!(Config::parse(&[]).is_ok());
}
