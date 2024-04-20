#[derive(Debug, Clone, PartialEq)]
pub enum OpCodes {
    Add(u8),
    Sub(u8),
    Inc(u8),
    Dec(u8),
    Output,
    Input,
    Loop(Vec<OpCodes>),
    // Special optimizations
    Clear,
}
