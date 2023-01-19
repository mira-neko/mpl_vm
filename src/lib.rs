mod instruction;

pub use instruction::Instructions;
use std::fmt;

#[derive(Debug, Clone)]
pub struct State<F: FnMut() -> Option<f64>> {
    ip: usize,
    stack: Vec<f64>,
    call_stack: Vec<usize>,
    array: [f64; 256],
    ap: u8,
    input: F,
    debug: bool,
}

#[derive(Debug, Clone)]
pub struct Program<F: FnMut() -> Option<f64>> {
    program: Vec<Instructions>,
    state: State<F>,
    finished: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    PopFromEmptyStack,
    PopFromEmptyCallStack,
    InputFailed,
}

impl<F: FnMut() -> Option<f64>> State<F> {
    fn new(input: F, debug: bool) -> State<F> {
        State {
            stack: Vec::new(),
            call_stack: Vec::new(),
            array: [0.; 256],
            ap: 0,
            ip: 0,
            input,
            debug,
        }
    }
}

impl<F: FnMut() -> Option<f64>> From<(Vec<Instructions>, F)> for Program<F> {
    fn from((program, input): (Vec<Instructions>, F)) -> Program<F> {
        Program {
            program,
            state: State::new(input, false),
            finished: false,
        }
    }
}

impl<F: FnMut() -> Option<f64>> From<(Vec<Instructions>, F, bool)> for Program<F> {
    fn from((program, input, debug): (Vec<Instructions>, F, bool)) -> Program<F> {
        Program {
            program,
            state: State::new(input, debug),
            finished: false,
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
            let res = self.program[self.state.ip].eval(&mut self.state);

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
