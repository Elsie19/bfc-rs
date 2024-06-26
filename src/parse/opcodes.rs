use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum OpCodes {
    Add(u32),
    Sub(u32),
    Inc(u32),
    Dec(u32),
    Output,
    Input,
    Loop(Vec<Tokens>),
    // Special optimizations
    Clear,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tokens {
    code: OpCodes,
    /// line, column
    location: (u32, u32),
}

impl Tokens {
    pub fn new(code: OpCodes, location: (u32, u32)) -> Self {
        Tokens { code, location }
    }

    pub fn get_type(&self) -> &OpCodes {
        &self.code
    }

    pub fn get_location(&self) -> (u32, u32) {
        self.location
    }
}

impl fmt::Display for OpCodes {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Add(x) => write!(fmt, "{:+<1$}", "", *x as usize).unwrap(),
            Self::Sub(x) => write!(fmt, "{:-<1$}", "", *x as usize).unwrap(),
            Self::Inc(x) => write!(fmt, "{:><1$}", "", *x as usize).unwrap(),
            Self::Dec(x) => write!(fmt, "{:<<1$}", "", *x as usize).unwrap(),
            Self::Output => write!(fmt, ".").unwrap(),
            Self::Input => write!(fmt, ",").unwrap(),
            Self::Clear => write!(fmt, "[-]").unwrap(),
            Self::Loop(x) => {
                write!(fmt, "[").unwrap();
                for item in x {
                    write!(fmt, "{}", item.get_type()).unwrap();
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
