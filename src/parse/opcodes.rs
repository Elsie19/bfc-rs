use core::fmt;

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

impl fmt::Display for OpCodes {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Add(x) => {
                write!(fmt, "{}", format!("{:+<1$}", "", usize::from(*x))).unwrap();
            }
            Self::Sub(x) => {
                write!(fmt, "{}", format!("{:-<1$}", "", usize::from(*x))).unwrap();
            }
            Self::Inc(x) => {
                write!(fmt, "{}", format!("{:><1$}", "", usize::from(*x))).unwrap();
            }
            Self::Dec(x) => {
                write!(fmt, "{}", format!("{:<<1$}", "", usize::from(*x))).unwrap();
            }
            Self::Output => {
                write!(fmt, ".").unwrap();
            }
            Self::Input => {
                write!(fmt, ",").unwrap();
            }
            Self::Clear => {
                write!(fmt, "[-]").unwrap();
            }
            Self::Loop(x) => {
                write!(fmt, "[").unwrap();
                for item in x {
                    write!(fmt, "{}", item).unwrap();
                }
                write!(fmt, "]").unwrap();
            }
        }
        Ok(())
    }
}

impl OpCodes {
    pub fn opposite(&self) -> Option<Self> {
        match self {
            OpCodes::Add(x) => Some(OpCodes::Sub(*x)),
            OpCodes::Sub(x) => Some(OpCodes::Add(*x)),
            OpCodes::Inc(x) => Some(OpCodes::Dec(*x)),
            OpCodes::Dec(x) => Some(OpCodes::Inc(*x)),
            _ => None,
        }
    }
}
