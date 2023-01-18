use std::{
    fmt,
    io::{stdin, stdout, Write},
};

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

#[derive(Debug, Clone)]
pub struct Program {
    program: Vec<Instructions>,
    ip: usize,
    stack: Vec<f64>,
    call_stack: Vec<usize>,
    array: [f64; 256],
    ap: u8,
    debug: bool,
}

#[derive(Debug)]
pub enum Error {
    PopFromEmptyStack,
    PopFromEmptyCallStack,
    InputFailed,
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
    pub fn eval(
        &self,
        ip: &mut usize,
        stack: &mut Vec<f64>,
        call_stack: &mut Vec<usize>,
        array: &mut [f64; 256],
        ap: &mut u8,
        debug: bool,
    ) -> Result<Option<f64>, Error> {
        use Instructions::*;

        match self {
            Psh(n) => stack.push(*n),
            Pfa => stack.push(array[*ap as usize]),
            Pta => array[*ap as usize] = stack.pop().ok_or(Error::PopFromEmptyStack)?,
            Gap => stack.push(*ap as f64),
            Ptap => *ap = stack.pop().ok_or(Error::PopFromEmptyStack)? as u8,
            Sap(n) => *ap = *n,
            Pek => return Some(stack.last().copied().ok_or(Error::PopFromEmptyStack)).transpose(),
            Inp => {
                let mut input = String::new();
                print!("enter number: ");
                stdout().flush().or(Err(Error::InputFailed))?;
                stdin().read_line(&mut input).or(Err(Error::InputFailed))?;
                stack.push(
                    input[0..(input.len() - 1)]
                        .parse::<f64>()
                        .or(Err(Error::InputFailed))?,
                )
            }
            Dup => stack.push(*stack.last().ok_or(Error::PopFromEmptyStack)?),
            Pop => stack.pop().map(|_| ()).ok_or(Error::PopFromEmptyStack)?,
            Swp => {
                let len = stack.len();
                stack.swap(len - 1, len - 2)
            }
            Lsw(n) => {
                let len = stack.len();
                stack.swap(len - 1, len - *n - 2)
            }
            Add => {
                let snd = stack.pop().ok_or(Error::PopFromEmptyStack)?;
                let fst = stack.pop().ok_or(Error::PopFromEmptyStack)?;
                stack.push(fst + snd)
            }
            Sub => {
                let snd = stack.pop().ok_or(Error::PopFromEmptyStack)?;
                let fst = stack.pop().ok_or(Error::PopFromEmptyStack)?;
                stack.push(fst - snd)
            }
            Mul => {
                let snd = stack.pop().ok_or(Error::PopFromEmptyStack)?;
                let fst = stack.pop().ok_or(Error::PopFromEmptyStack)?;
                stack.push(fst * snd)
            }
            Div => {
                let snd = stack.pop().ok_or(Error::PopFromEmptyStack)?;
                let fst = stack.pop().ok_or(Error::PopFromEmptyStack)?;
                stack.push(fst / snd)
            }
            Mod => {
                let snd = stack.pop().ok_or(Error::PopFromEmptyStack)?;
                let fst = stack.pop().ok_or(Error::PopFromEmptyStack)?;
                stack.push(fst % snd)
            }
            Abs => {
                let n = stack.pop().ok_or(Error::PopFromEmptyStack)?;
                stack.push(n.abs())
            }
            Max => {
                let snd = stack.pop().ok_or(Error::PopFromEmptyStack)?;
                let fst = stack.pop().ok_or(Error::PopFromEmptyStack)?;
                stack.push(fst.max(snd))
            }
            Min => {
                let snd = stack.pop().ok_or(Error::PopFromEmptyStack)?;
                let fst = stack.pop().ok_or(Error::PopFromEmptyStack)?;
                stack.push(fst.min(snd))
            }
            Jmp(n) => {
                call_stack.push(*ip);
                *ip = n.overflowing_sub(1).0
            }
            Jiz(n) => {
                if stack.pop().ok_or(Error::PopFromEmptyStack)?.abs() <= 1E-7 {
                    call_stack.push(*ip);
                    *ip = n.overflowing_sub(1).0
                }
            }
            Jnz(n) => {
                if stack.pop().ok_or(Error::PopFromEmptyStack)?.abs() >= 1E-7 {
                    call_stack.push(*ip);
                    *ip = n.overflowing_sub(1).0
                }
            }
            Ipta => array[*ap as usize] = *ip as f64,
            Jmpa => {
                call_stack.push(*ip);
                *ip = array[*ap as usize] as usize
            }
            Jiza => {
                if stack.pop().ok_or(Error::PopFromEmptyStack)?.abs() <= 1E-7 {
                    call_stack.push(*ip);
                    *ip = array[*ap as usize] as usize
                }
            }
            Jnza => {
                if stack.pop().ok_or(Error::PopFromEmptyStack)?.abs() >= 1E-7 {
                    call_stack.push(*ip);
                    *ip = array[*ap as usize] as usize
                }
            }
            Ret => {
                *ip = call_stack.pop().ok_or(Error::PopFromEmptyCallStack)?;
            }
        };
        *ip = ip.overflowing_add(1).0;

        if debug {
            println!("instruction: {self:?}\nip: {ip}\nstack: {stack:?}\ncall_stack: {call_stack:?}\narray: {array:?}\nap: {ap}\n");
        }

        Ok(None)
    }
}

impl From<Vec<Instructions>> for Program {
    fn from(program: Vec<Instructions>) -> Program {
        Program {
            program,
            stack: Vec::new(),
            call_stack: Vec::new(),
            array: [0.; 256],
            ap: 0,
            ip: 0,
            debug: false,
        }
    }
}

impl From<(Vec<Instructions>, bool)> for Program {
    fn from((program, debug): (Vec<Instructions>, bool)) -> Program {
        Program {
            program,
            stack: Vec::new(),
            call_stack: Vec::new(),
            array: [0.; 256],
            ap: 0,
            ip: 0,
            debug,
        }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.program
            .iter()
            .try_for_each(|instruction| writeln!(f, "{instruction}"))
    }
}

impl Iterator for Program {
    type Item = Result<Option<f64>, Error>;

    fn next(&mut self) -> Option<Result<Option<f64>, Error>> {
        if self.ip < self.program.len() {
            Some(self.program[self.ip].eval(
                &mut self.ip,
                &mut self.stack,
                &mut self.call_stack,
                &mut self.array,
                &mut self.ap,
                self.debug,
            ))
        } else {
            None
        }
    }
}
