#[derive(Debug)]
pub enum Csv {
    FileReadError(String),
    ParseError(Vec<csv::Error>),
}
