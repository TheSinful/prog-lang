#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Eol,                 // end of line
    Add,                 // +
    Subtract,            // -
    Divide,              // "/"
    Multiply,            // "*"
    SingleComment,       // "//"
    MutVarDeclaration,   // "set"
    ImmutVarDeclaration, // "const"
    Assignment,          // =
    Int(i32),
    Variable(String),
}

// set x =
