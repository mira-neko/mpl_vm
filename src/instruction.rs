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

impl Instructions {
    pub fn eval<F: FnMut() -> Option<f64>>(
        &self,
        state: &mut super::State<F>,
    ) -> Result<Option<f64>, super::Error> {
        use Instructions::*;

        let mut res = None;

        match self {
            Psh(n) => state.stack.push(*n),
            Pfa => state.stack.push(state.array[state.ap as usize]),
            Pta => state.array[state.ap as usize] = state.stack.pop().ok_or(super::Error::PopFromEmptyStack)?,
            Gap => state.stack.push(state.ap as f64),
            Ptap => state.ap = state.stack.pop().ok_or(super::Error::PopFromEmptyStack)? as u8,
            Sap(n) => state.ap = *n,
            Pek => res = Some(state.stack.last().copied().ok_or(super::Error::PopFromEmptyStack)?),
            Inp => state.stack.push((state.input)().ok_or(super::Error::InputFailed)?),
            Dup => state.stack.push(*state.stack.last().ok_or(super::Error::PopFromEmptyStack)?),
            Pop => state.stack.pop().map(|_| ()).ok_or(super::Error::PopFromEmptyStack)?,
            Swp => {
                let len = state.stack.len();
                state.stack.swap(len - 1, len - 2)
            }
            Lsw(n) => {
                let len = state.stack.len();
                state.stack.swap(len - 1, len - *n - 2)
            }
            Add => {
                let snd = state.stack.pop().ok_or(super::Error::PopFromEmptyStack)?;
                let fst = state.stack.pop().ok_or(super::Error::PopFromEmptyStack)?;
                state.stack.push(fst + snd)
            }
            Sub => {
                let snd = state.stack.pop().ok_or(super::Error::PopFromEmptyStack)?;
                let fst = state.stack.pop().ok_or(super::Error::PopFromEmptyStack)?;
                state.stack.push(fst - snd)
            }
            Mul => {
                let snd = state.stack.pop().ok_or(super::Error::PopFromEmptyStack)?;
                let fst = state.stack.pop().ok_or(super::Error::PopFromEmptyStack)?;
                state.stack.push(fst * snd)
            }
            Div => {
                let snd = state.stack.pop().ok_or(super::Error::PopFromEmptyStack)?;
                let fst = state.stack.pop().ok_or(super::Error::PopFromEmptyStack)?;
                state.stack.push(fst / snd)
            }
            Mod => {
                let snd = state.stack.pop().ok_or(super::Error::PopFromEmptyStack)?;
                let fst = state.stack.pop().ok_or(super::Error::PopFromEmptyStack)?;
                state.stack.push(fst % snd)
            }
            Abs => {
                let n = state.stack.pop().ok_or(super::Error::PopFromEmptyStack)?;
                state.stack.push(n.abs())
            }
            Max => {
                let snd = state.stack.pop().ok_or(super::Error::PopFromEmptyStack)?;
                let fst = state.stack.pop().ok_or(super::Error::PopFromEmptyStack)?;
                state.stack.push(fst.max(snd))
            }
            Min => {
                let snd = state.stack.pop().ok_or(super::Error::PopFromEmptyStack)?;
                let fst = state.stack.pop().ok_or(super::Error::PopFromEmptyStack)?;
                state.stack.push(fst.min(snd))
            }
            Jmp(n) => {
                state.call_stack.push(state.ip);
                state.ip = n.overflowing_sub(1).0
            }
            Jiz(n) => {
                if state.stack.pop().ok_or(super::Error::PopFromEmptyStack)?.abs() <= 1E-7 {
                    state.call_stack.push(state.ip);
                    state.ip = n.overflowing_sub(1).0
                }
            }
            Jnz(n) => {
                if state.stack.pop().ok_or(super::Error::PopFromEmptyStack)?.abs() >= 1E-7 {
                    state.call_stack.push(state.ip);
                    state.ip = n.overflowing_sub(1).0
                }
            }
            Ipta => state.array[state.ap as usize] = state.ip as f64,
            Jmpa => {
                state.call_stack.push(state.ip);
                state.ip = state.array[state.ap as usize] as usize
            }
            Jiza => {
                if state.stack.pop().ok_or(super::Error::PopFromEmptyStack)?.abs() <= 1E-7 {
                    state.call_stack.push(state.ip);
                    state.ip = state.array[state.ap as usize] as usize
                }
            }
            Jnza => {
                if state.stack.pop().ok_or(super::Error::PopFromEmptyStack)?.abs() >= 1E-7 {
                    state.call_stack.push(state.ip);
                    state.ip = state.array[state.ap as usize] as usize
                }
            }
            Ret => {
                state.ip = state.call_stack.pop().ok_or(super::Error::PopFromEmptyCallStack)?;
            }
        };
        state.ip = state.ip.overflowing_add(1).0;

        if state.debug {
            println!(
                "instruction: {self:?}\nip: {}\nstack: {:?}\ncall_stack: {:?}\narray: {:?}\nap: {}\n",
                state.ip,
                state.stack,
                state.call_stack,
                state.array,
                state.ap,
            );
        }

        Ok(res)
    }
}
