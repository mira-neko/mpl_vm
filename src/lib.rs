mod instruction;
mod state;

pub use instruction::Instructions;
pub use state::State;
use std::fmt;

#[derive(Clone)]
pub struct Program<F: FnMut() -> Option<f64>> {
    program: Vec<Instructions>,
    state: State,
    input: F,
    debug: bool,
    finished: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    PopFromEmptyStack,
    PopFromEmptyCallStack,
    InputFailed,
}

impl<F: FnMut() -> Option<f64>> From<(Vec<Instructions>, F)> for Program<F> {
    fn from((program, input): (Vec<Instructions>, F)) -> Program<F> {
        Program {
            program,
            state: State::default(),
            finished: false,
            input,
            debug: false,
        }
    }
}

impl<F: FnMut() -> Option<f64>> From<(Vec<Instructions>, F, bool)> for Program<F> {
    fn from((program, input, debug): (Vec<Instructions>, F, bool)) -> Program<F> {
        Program {
            program,
            state: State::default(),
            finished: false,
            input,
            debug,
        }
    }
}

impl<F: FnMut() -> Option<f64>> fmt::Display for Program<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.program
            .iter()
            .try_for_each(|instruction| writeln!(f, "{instruction}"))
    }
}

impl<F: FnMut() -> Option<f64>> Iterator for Program<F> {
    type Item = Result<Option<f64>, Error>;

    fn next(&mut self) -> Option<Result<Option<f64>, Error>> {
        if !self.finished && self.state.ip < self.program.len() {
            let res = self
                .state
                .eval(&self.program[self.state.ip], &mut self.input, self.debug);

            if res.is_err() {
                self.finished = true;
            }

            Some(res)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_test() {
        use Instructions::*;

        let mut program = Program::from((vec![Inp, Psh(1.), Add, Pek, Pop, Pop], || Some(5.)));

        assert_eq!(program.next(), Some(Ok(None)));
        assert_eq!(program.next(), Some(Ok(None)));
        assert_eq!(program.next(), Some(Ok(None)));
        assert_eq!(program.next(), Some(Ok(Some(6.))));
        assert_eq!(program.next(), Some(Ok(None)));
        assert_eq!(program.next(), Some(Err(Error::PopFromEmptyStack)));
        assert_eq!(program.next(), None);
    }

    #[test]
    fn test_factorial() {
        use Instructions::*;

        let mut program = Program::from((
            vec![
                Psh(1.),
                Psh(1.),
                Max,
                Inp,
                Dup,
                Lsw(1),
                Mul,
                Swp,
                Psh(1.),
                Sub,
                Dup,
                Jnz(4),
                Pop,
                Pek,
            ],
            || Some(5.),
        ));

        assert_eq!(program.next(), Some(Ok(None)));
        assert_eq!(program.next(), Some(Ok(None)));
        assert_eq!(program.next(), Some(Ok(None)));
        assert_eq!(program.next(), Some(Ok(None)));
        for _ in 0..(5 * 8) {
            assert_eq!(program.next(), Some(Ok(None)));
        }
        assert_eq!(program.next(), Some(Ok(None)));
        assert_eq!(program.next(), Some(Ok(Some(120.))));
        assert_eq!(program.next(), None);
    }
}
