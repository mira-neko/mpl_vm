use std::{io::{stdin, stdout, Write}, fmt};

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
    Ret
}

#[derive(Debug, Clone)]
pub struct Program(Vec<Instructions>);

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
        debug: bool
    ) -> Option<()> {
        use Instructions::*;

        match self {
            Psh(n) => stack.push(*n),
            Pfa => stack.push(array[*ap as usize]),
            Pta => array[*ap as usize] = stack.pop()?,
            Gap => stack.push(*ap as f64),
            Ptap => *ap = stack.pop()? as u8,
            Sap(n) => *ap = *n,
            Pek => println!("{}", stack.last()?),
            Inp => {
                let mut input = String::new();
                print!("enter number: ");
                stdout().flush().ok()?;
                stdin().read_line(&mut input).ok()?;
                stack.push(input[0..(input.len()-1)].parse::<f64>().ok()?)
            }
            Dup => stack.push(*stack.last()?),
            Pop => stack.pop().map(|_| ())?,
            Swp => {
                let len = stack.len();
                stack.swap(len - 1, len - 2)
            }
            Lsw(n) => {
                let len = stack.len();
                stack.swap(len - 1, len - *n - 2)
            }
            Add => {
                let snd = stack.pop()?;
                let fst = stack.pop()?;
                stack.push(fst + snd)
            }
            Sub => {
                let snd = stack.pop()?;
                let fst = stack.pop()?;
                stack.push(fst - snd)
            }
            Mul => {
                let snd = stack.pop()?;
                let fst = stack.pop()?;
                stack.push(fst * snd)
            }
            Div => {
                let snd = stack.pop()?;
                let fst = stack.pop()?;
                stack.push(fst / snd)
            }
            Mod => {
                let snd = stack.pop()?;
                let fst = stack.pop()?;
                stack.push(fst % snd)
            }
            Abs => {
                let n = stack.pop()?;
                stack.push(n.abs())
            }
            Max => {
                let snd = stack.pop()?;
                let fst = stack.pop()?;
                stack.push(fst.max(snd))
            }
            Min => {
                let snd = stack.pop()?;
                let fst = stack.pop()?;
                stack.push(fst.min(snd))
            }
            Jmp(n) => {
                call_stack.push(*ip);
                *ip = n.overflowing_sub(1).0
            }
            Jiz(n) => {
                if stack.pop()?.abs() <= 1E-7 {
                    call_stack.push(*ip);
                    *ip = n.overflowing_sub(1).0
                }
            }
            Jnz(n) => {
                if stack.pop()?.abs() >= 1E-7 {
                    call_stack.push(*ip);
                    *ip = n.overflowing_sub(1).0
                }
            }
            Ipta => {
                array[*ap as usize] = *ip as f64
            }
            Jmpa => {
                call_stack.push(*ip);
                *ip = array[*ap as usize] as usize
            }
            Jiza => {
                if stack.pop()?.abs() <= 1E-7 {
                    call_stack.push(*ip);
                    *ip = array[*ap as usize] as usize
                }
            }
            Jnza => {
                if stack.pop()?.abs() >= 1E-7 {
                    call_stack.push(*ip);
                    *ip = array[*ap as usize] as usize
                }
            }
            Ret => {
                *ip = call_stack.pop()?;
            }
        };
        *ip = ip.overflowing_add(1).0;

        if debug {
            println!("instruction: {self:?}\nip: {ip}\nstack: {stack:?}\ncall_stack: {call_stack:?}\narray: {array:?}\nap: {ap}\n");
        }

        Some(())
    }
}

impl From<Vec<Instructions>> for Program {
    fn from(program: Vec<Instructions>) -> Program {
        Program(program)
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for instruction in &self.0 {
            writeln!(f, "{instruction}")?;
        }

        Ok(())
    }
}

impl Program {
    pub fn eval(self, debug: bool) -> Option<()> {
        let mut stack = Vec::new();
        let mut call_stack = Vec::new();
        let mut array = [0.; 256];
        let mut ap = 0;
        let mut ip = 0;

        while ip < self.0.len() {
            self.0[ip].eval(&mut ip, &mut stack, &mut call_stack, &mut array, &mut ap, debug)?;
        }

        Some(())
    }
}
