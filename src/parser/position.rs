
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Position {
    pub start: usize,
    pub end: usize,
    pub line_number: usize,
    pub column_number: usize
}