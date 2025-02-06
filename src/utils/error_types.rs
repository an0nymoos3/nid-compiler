#[allow(dead_code)]
pub enum TokenizerError {
    EmptySourceCode,
    UnexpectedChar,
}

#[allow(dead_code)]
pub enum ParserError {
    InvalidIdentifier,
    InvalidOperator,

    MissingToken,
    MissingEol,
    MissingOpenScope,
    MissingOpenParen,

    ParserFailure, // General error indicating at least one step in the parser failed

    UnknownVarOrVal,
}

#[allow(dead_code)]
pub enum CodeGenError {}

#[allow(dead_code)]
pub enum AssemblerError {
    UnknownInst,
}
