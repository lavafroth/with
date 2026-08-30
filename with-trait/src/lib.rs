pub trait WithOpt: Sized {
    fn parse(args: &[&str]) -> Result<Self, String>;
    fn parse_args() -> Result<Self, String> {
        let args = std::env::args().collect::<Vec<_>>();
        let args_strref = args.iter().map(|s| s.as_str()).collect::<Vec<_>>();
        Self::parse(args_strref.as_slice())
    }
}
