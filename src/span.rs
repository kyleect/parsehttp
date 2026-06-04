use std::ops::Range;

use bon::bon;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: u32,
    pub end: u32,
    pub line: u32,
    pub column: u32,
}

#[bon]
impl Span {
    #[builder]
    pub fn new(span: Range<u32>, line: u32, column: u32) -> Self {
        Self {
            start: span.start,
            end: span.end,
            line,
            column,
        }
    }

    pub fn slice<'a>(&self, src: &'a str) -> &'a str {
        &src[self.start as usize..self.end as usize]
    }
}
