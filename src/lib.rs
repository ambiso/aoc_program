pub type Cell = u32;
pub type Mem = Vec<Cell>;
use std::io::BufRead;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MachineError {
    #[error("Invalid Opcode")]
    InvalidOpCode,
    #[error("Invalid parameter mode")]
    InvalidParameterMode,
    #[error("Missing input")]
    IOMissingInput,
    #[error("I/O failed")]
    IOFailed,
    #[error("Could not parse input")]
    IOParse,
}

fn get_param(mem: &Mem, ip: usize, param: usize, param_mode: u32) -> Result<u32, MachineError> {
    if param_mode == 0 {
        // position mode
        return Ok(mem[mem[ip + param] as usize]);
    }
    if param_mode == 1 {
        // immediate mode
        return Ok(mem[ip + param]);
    }
    Err(MachineError::InvalidParameterMode)
}

pub trait IO {
    fn input(&mut self) -> Result<Cell, MachineError>;
    fn output(&mut self, v: Cell) -> Result<(), MachineError>;
}

pub struct LineIO;

impl IO for LineIO {
    fn input(&mut self) -> Result<Cell, MachineError> {
        let stdin = std::io::stdin();
        let input = stdin.lock().lines().next();
        if input.is_none() {
            return Err(MachineError::IOMissingInput);
        }
        let s = input.unwrap().map_err(|_| MachineError::IOFailed)?;
        s.parse().map_err(|_| MachineError::IOParse)
    }

    fn output(&mut self, v: Cell) -> Result<(), MachineError> {
        println!("{}", v);
        Ok(())
    }
}

pub struct EmptyIO {}

impl IO for EmptyIO {
    fn input(&mut self) -> Result<Cell, MachineError> {
        Err(MachineError::IOMissingInput)
    }

    fn output(&mut self, _v: Cell) -> Result<(), MachineError> {
        Ok(())
    }
}

pub struct VecIO {
    pos: usize,
    input: Vec<Cell>,
    output: Vec<Cell>,
}

impl VecIO {
    pub fn new(input: Vec<Cell>, output: Vec<Cell>) -> Self {
        Self {
            pos: 0,
            input: input,
            output: output,
        }
    }
}

impl IO for VecIO {
    fn input(&mut self) -> Result<Cell, MachineError> {
        if self.pos < self.input.len() {
            let rv = Ok(self.input[self.pos]);
            self.pos += 1;
            rv
        } else {
            Err(MachineError::IOMissingInput)
        }
    }

    fn output(&mut self, v: Cell) -> Result<(), MachineError> {
        self.output.push(v);
        Ok(())
    }
}

pub const OP_ADD: Cell = 1;
pub const OP_MUL: Cell = 2;
pub const OP_INPUT: Cell = 3;
pub const OP_OUTPUT: Cell = 4;
pub const OP_HALT: Cell = 99;

pub fn execute<'a>(mem: &'a mut Mem, io: &mut impl IO) -> Result<&'a mut Mem, MachineError> {
    let mut ip = 0;
    loop {
        let op = mem[ip];
        let mode1 = (op / 100) % 10;
        let mode2 = (op / 1000) % 10;
        let op = op % 100;
        // let mode3 = (op / 10000) % 10;
        match op {
            OP_ADD | OP_MUL => {
                let a = get_param(mem, ip, 1, mode1)?;
                let b = get_param(mem, ip, 2, mode2)?;
                let tgt = mem[ip + 3] as usize;
                mem[tgt] = match op {
                    OP_ADD => a + b,
                    OP_MUL => a * b,
                    _ => unreachable!(),
                };
                ip += 4;
            }
            OP_INPUT => {
                let tgt = mem[ip + 1] as usize;
                mem[tgt] = io.input()?;
                ip += 2;
            }
            OP_OUTPUT => {
                let a = get_param(mem, ip, 1, mode1)?;
                io.output(a)?;
                ip += 2;
            }
            OP_HALT => {
                break;
            }
            _ => {
                println!("{:?}", mem);
                return Err(MachineError::InvalidOpCode);
            }
        }
    }
    Ok(mem)
}

pub fn parse_mem() -> Vec<u32> {
    let stdin = std::io::stdin();
    return stdin
        .lock()
        .split(b',')
        .map(|op| {
            let op = op.unwrap();
            let op = std::str::from_utf8(&op).unwrap().trim();
            op.parse().unwrap()
        })
        .collect();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(
            execute(&mut vec![1, 0, 0, 0, 99], &mut EmptyIO {}).unwrap(),
            &mut vec![2, 0, 0, 0, 99]
        );
    }

    #[test]
    fn test_mul() {
        assert_eq!(
            execute(&mut vec![2, 3, 0, 3, 99], &mut EmptyIO {}).unwrap(),
            &mut vec![2, 3, 0, 6, 99]
        );
    }

    #[test]
    fn test_mul2() {
        assert_eq!(
            execute(&mut vec![2, 4, 4, 5, 99, 0], &mut EmptyIO {}).unwrap(),
            &mut vec![2, 4, 4, 5, 99, 9801]
        );
    }

    #[test]
    fn test_add2() {
        assert_eq!(
            execute(&mut vec![1, 1, 1, 4, 99, 5, 6, 0, 99], &mut EmptyIO {}).unwrap(),
            &mut vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }

    #[test]
    fn test_param_mode() {
        assert_eq!(
            execute(&mut vec![1002, 4, 3, 4, 33], &mut EmptyIO {}).unwrap(),
            &mut vec![1002, 4, 3, 4, 99]
        );
    }
}
