use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum ParseError {
        ExpectedToken(token: String) {}
        ExpectedTokens(tokens: &'static [&'static str]) {}
        InvalidOperator(op: String) {}
        UnexpectedCharacter(ch: char) {}
        EOF {}
    }
}
