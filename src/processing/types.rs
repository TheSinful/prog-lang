use super::syntax_elements::Token;

pub type VariableName = String;
pub type VecPosition = i8;
pub type LineTokenizedBody = Vec<Token>;
pub type LineBaseBody = String;
pub type LineSplitBody = Vec<String>;
pub type LineNumber = i64;

#[derive(Clone, Default)]
pub struct Line {
    pub tokenized_body: LineTokenizedBody,
    pub base_body: LineBaseBody,
    pub split_body: LineSplitBody,
    pub number: LineNumber,
}

impl Line {
    pub fn new(
        tokenized_body: Vec<Token>,
        base_body: LineBaseBody,
        split_body: LineSplitBody,
        number: LineNumber,
    ) -> Line {
        Line {
            tokenized_body,
            base_body,
            split_body,
            number,
        }
    }
}
