pub enum TokenizerError {
    EmptySourceCode,
    UnexpectedChar,
}

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

pub enum CodeGenError {}

pub enum AssemblerError {
    UnknownInst,
}
