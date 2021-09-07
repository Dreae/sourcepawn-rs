pub struct CodeContext {
    pub filename: String,
    pub line: usize,
    pub column: usize
}

#[allow(dead_code)]
struct MacroExpansion {
    pos: usize,
    tokens: usize
}

#[allow(dead_code)]
struct SourceFile {
    filename: String,
    tokens: usize,
    expansions: Vec<MacroExpansion>
}

pub trait PreprocessorContext {
    fn token_pos_to_src_descriptor(&self, token_pos: usize) -> CodeContext;
}
