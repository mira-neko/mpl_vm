use std::fmt;

#[derive(Debug, Clone)]
pub enum Instructions {
    Psh(f64),
    Pfa,
    Pta,
    Gap,
    Ptap,
    Sap(u8),
    Pek,
    Inp,
    Dup,
    Pop,
    Swp,
    Lsw(usize),
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Abs,
    Max,
    Min,
    Jmp(usize),
    Jiz(usize),
    Jnz(usize),
    Ipta,
    Jmpa,
    Jiza,
    Jnza,
    Ret,
}

impl fmt::Display for Instructions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Instructions::*;

        match self {
            Psh(n) => write!(f, "psh {n}"),
            Pfa => write!(f, "pfa"),
            Pta => write!(f, "pta"),
            Gap => write!(f, "gap"),
            Ptap => write!(f, "ptap"),
            Sap(n) => write!(f, "sap {n}"),
            Pek => write!(f, "pek"),
            Inp => write!(f, "inp"),
            Dup => write!(f, "dup"),
            Pop => write!(f, "pop"),
            Swp => write!(f, "swp"),
            Lsw(n) => write!(f, "lsw {n}"),
            Add => write!(f, "add"),
            Sub => write!(f, "sub"),
            Mul => write!(f, "mul"),
            Div => write!(f, "div"),
            Mod => write!(f, "mod"),
            Abs => write!(f, "abs"),
            Max => write!(f, "max"),
            Min => write!(f, "min"),
            Jmp(n) => write!(f, "jmp {n}"),
            Jiz(n) => write!(f, "jiz {n}"),
            Jnz(n) => write!(f, "jnz {n}"),
            Ipta => write!(f, "ipta"),
            Jmpa => write!(f, "jmpa"),
            Jiza => write!(f, "jiza"),
            Jnza => write!(f, "jnza"),
            Ret => write!(f, "ret"),
        }
    }
}
