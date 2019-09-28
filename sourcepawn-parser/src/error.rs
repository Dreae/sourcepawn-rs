pub struct ParserError {
    pub message: String,
}

impl ParserError {
    pub fn new(message: String) -> ParserError {
        ParserError { message }
    }
}
