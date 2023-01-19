#[derive(Clone)]
pub struct State {
    pub(super) ip: usize,
    pub(super) stack: Vec<f64>,
    pub(super) call_stack: Vec<usize>,
    pub(super) array: [f64; 256],
    pub(super) ap: u8,
}

impl Default for State {
    fn default() -> State {
        State {
            ip: 0,
            stack: Vec::new(),
            call_stack: Vec::new(),
            array: [0.; 256],
            ap: 0,
        }
    }
}

impl State {
    pub fn eval<F: FnMut() -> Option<f64>>(
        &mut self,
        instruction: &super::Instructions,
        input: &mut F,
        debug: bool,
    ) -> Result<Option<f64>, super::Error> {
        use super::Instructions::*;

        let mut res = None;

        match instruction {
            Psh(n) => self.stack.push(*n),
            Pfa => self.stack.push(self.array[self.ap as usize]),
            Pta => {
                self.array[self.ap as usize] =
                    self.stack.pop().ok_or(super::Error::PopFromEmptyStack)?
            }
            Gap => self.stack.push(self.ap as f64),
            Ptap => self.ap = self.stack.pop().ok_or(super::Error::PopFromEmptyStack)? as u8,
            Sap(n) => self.ap = *n,
            Pek => {
                res = Some(
                    self.stack
                        .last()
                        .copied()
                        .ok_or(super::Error::PopFromEmptyStack)?,
                )
            }
            Inp => self.stack.push((input)().ok_or(super::Error::InputFailed)?),
            Dup => self
                .stack
                .push(*self.stack.last().ok_or(super::Error::PopFromEmptyStack)?),
            Pop => self
                .stack
                .pop()
                .map(|_| ())
                .ok_or(super::Error::PopFromEmptyStack)?,
            Swp => {
                let len = self.stack.len();
                self.stack.swap(len - 1, len - 2)
            }
            Lsw(n) => {
                let len = self.stack.len();
                self.stack.swap(len - 1, len - *n - 2)
            }
            Add => {
                let snd = self.stack.pop().ok_or(super::Error::PopFromEmptyStack)?;
                let fst = self.stack.pop().ok_or(super::Error::PopFromEmptyStack)?;
                self.stack.push(fst + snd)
            }
            Sub => {
                let snd = self.stack.pop().ok_or(super::Error::PopFromEmptyStack)?;
                let fst = self.stack.pop().ok_or(super::Error::PopFromEmptyStack)?;
                self.stack.push(fst - snd)
            }
            Mul => {
                let snd = self.stack.pop().ok_or(super::Error::PopFromEmptyStack)?;
                let fst = self.stack.pop().ok_or(super::Error::PopFromEmptyStack)?;
                self.stack.push(fst * snd)
            }
            Div => {
                let snd = self.stack.pop().ok_or(super::Error::PopFromEmptyStack)?;
                let fst = self.stack.pop().ok_or(super::Error::PopFromEmptyStack)?;
                self.stack.push(fst / snd)
            }
            Mod => {
                let snd = self.stack.pop().ok_or(super::Error::PopFromEmptyStack)?;
                let fst = self.stack.pop().ok_or(super::Error::PopFromEmptyStack)?;
                self.stack.push(fst % snd)
            }
            Abs => {
                let n = self.stack.pop().ok_or(super::Error::PopFromEmptyStack)?;
                self.stack.push(n.abs())
            }
            Max => {
                let snd = self.stack.pop().ok_or(super::Error::PopFromEmptyStack)?;
                let fst = self.stack.pop().ok_or(super::Error::PopFromEmptyStack)?;
                self.stack.push(fst.max(snd))
            }
            Min => {
                let snd = self.stack.pop().ok_or(super::Error::PopFromEmptyStack)?;
                let fst = self.stack.pop().ok_or(super::Error::PopFromEmptyStack)?;
                self.stack.push(fst.min(snd))
            }
            Jmp(n) => {
                self.call_stack.push(self.ip);
                self.ip = n.overflowing_sub(1).0
            }
            Jiz(n) => {
                if self
                    .stack
                    .pop()
                    .ok_or(super::Error::PopFromEmptyStack)?
                    .abs()
                    <= 1E-7
                {
                    self.call_stack.push(self.ip);
                    self.ip = n.overflowing_sub(1).0
                }
            }
            Jnz(n) => {
                if self
                    .stack
                    .pop()
                    .ok_or(super::Error::PopFromEmptyStack)?
                    .abs()
                    >= 1E-7
                {
                    self.call_stack.push(self.ip);
                    self.ip = n.overflowing_sub(1).0
                }
            }
            Ipta => self.array[self.ap as usize] = self.ip as f64,
            Jmpa => {
                self.call_stack.push(self.ip);
                self.ip = self.array[self.ap as usize] as usize
            }
            Jiza => {
                if self
                    .stack
                    .pop()
                    .ok_or(super::Error::PopFromEmptyStack)?
                    .abs()
                    <= 1E-7
                {
                    self.call_stack.push(self.ip);
                    self.ip = self.array[self.ap as usize] as usize
                }
            }
            Jnza => {
                if self
                    .stack
                    .pop()
                    .ok_or(super::Error::PopFromEmptyStack)?
                    .abs()
                    >= 1E-7
                {
                    self.call_stack.push(self.ip);
                    self.ip = self.array[self.ap as usize] as usize
                }
            }
            Ret => {
                self.ip = self
                    .call_stack
                    .pop()
                    .ok_or(super::Error::PopFromEmptyCallStack)?;
            }
        };
        self.ip = self.ip.overflowing_add(1).0;

        if debug {
            println!(
                "instruction: {instruction:?}\nip: {}\nstack: {:?}\ncall_stack: {:?}\narray: {:?}\nap: {}\n",
                self.ip,
                self.stack,
                self.call_stack,
                self.array,
                self.ap,
            );
        }

        Ok(res)
    }
}
