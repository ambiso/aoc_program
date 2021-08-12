pub type Cell = i64;
use std::{
    io::{BufRead, Lines, Write},
    ops::{Index, IndexMut},
};

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

struct Mem<'a> {
    v: &'a mut Vec<Cell>,
}

impl<'a> Mem<'a> {
    fn new(v: &'a mut Vec<Cell>) -> Self {
        Self { v }
    }
}

impl<'a> Index<usize> for Mem<'a> {
    type Output = Cell;

    fn index(&self, index: usize) -> &Self::Output {
        self.v.get(index).unwrap_or(&0)
    }
}

impl<'a> IndexMut<usize> for Mem<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.v.len() {
            self.v.extend(vec![0; index - self.v.len() + 1]);
        }
        &mut self.v[index]
    }
}

fn get_param(
    mem: &Mem,
    ip: usize,
    rel_base: usize,
    param: usize,
    param_mode: Cell,
) -> Result<Cell, MachineError> {
    if param_mode == 0 {
        // position mode
        return Ok(mem[mem[ip + param] as usize]);
    }
    if param_mode == 1 {
        // immediate mode
        return Ok(mem[ip + param]);
    }
    if param_mode == 2 {
        // Relative mode
        return Ok(mem[(mem[ip + param] + rel_base as i64) as usize]);
    }
    Err(MachineError::InvalidParameterMode)
}

pub trait IO {
    fn input(&mut self) -> Result<Cell, MachineError>;
    fn output(&mut self, v: Cell) -> Result<(), MachineError>;
}

pub struct LineIO<B: BufRead, Wr: Write> {
    rd: Lines<B>,
    wr: Wr,
}

impl<B: BufRead, Wr: Write> LineIO<B, Wr> {
    pub fn new(rd: B, wr: Wr) -> Self {
        Self {
            rd: rd.lines(),
            wr: wr,
        }
    }
}

impl<Rd: BufRead, Wr: Write> IO for LineIO<Rd, Wr> {
    fn input(&mut self) -> Result<Cell, MachineError> {
        let input = self.rd.next();
        if input.is_none() {
            return Err(MachineError::IOMissingInput);
        }
        let s = input.unwrap().map_err(|_| MachineError::IOFailed)?;
        s.parse().map_err(|_| MachineError::IOParse)
    }

    fn output(&mut self, v: Cell) -> Result<(), MachineError> {
        writeln!(self.wr, "{}", v).map_err(|_| MachineError::IOFailed)?;
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
    pub pos: usize,
    pub input: Vec<Cell>,
    pub output: Vec<Cell>,
}

impl VecIO {
    pub fn new(input: Vec<Cell>) -> Self {
        Self {
            pos: 0,
            input: input,
            output: Vec::new(),
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

impl<T: IO + ?Sized> IO for Box<T> {
    fn input(&mut self) -> Result<Cell, MachineError> {
        (**self).input()
    }

    fn output(&mut self, v: Cell) -> Result<(), MachineError> {
        (**self).output(v)
    }
}

pub const OP_ADD: Cell = 1;
pub const OP_MUL: Cell = 2;
pub const OP_INPUT: Cell = 3;
pub const OP_OUTPUT: Cell = 4;
pub const OP_JT: Cell = 5;
pub const OP_JF: Cell = 6;
pub const OP_LT: Cell = 7;
pub const OP_EQ: Cell = 8;
pub const OP_ARB: Cell = 9;
pub const OP_HALT: Cell = 99;

pub fn execute<'a>(
    mem: &'a mut Vec<Cell>,
    io: &mut dyn IO,
) -> Result<&'a mut Vec<Cell>, MachineError> {
    let mut mem = Mem::new(mem);
    let mut ip = 0;
    let mut rel_base = 0;
    loop {
        let op = mem[ip];
        let mode1 = (op / 100) % 10;
        let mode2 = (op / 1000) % 10;
        // let mode3 = (op / 10000) % 10;
        let op = op % 100;
        println!("{}", op);
        match op {
            OP_ADD | OP_MUL => {
                let a = get_param(&mut mem, ip, rel_base, 1, mode1)?;
                let b = get_param(&mut mem, ip, rel_base, 2, mode2)?;
                let tgt = mem[ip + 3] as usize;
                println!("{} + {} => {}", a, b, tgt);
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
                let a = get_param(&mut mem, ip, rel_base, 1, mode1)?;
                io.output(a)?;
                println!("=> {}", a);
                ip += 2;
            }
            OP_JT => {
                let a = get_param(&mut mem, ip, rel_base, 1, mode1)?;
                let b = get_param(&mut mem, ip, rel_base, 2, mode2)?;
                if a != 0 {
                    ip = b as usize;
                } else {
                    ip += 3;
                }
            }
            OP_JF => {
                let a = get_param(&mut mem, ip, rel_base, 1, mode1)?;
                let b = get_param(&mut mem, ip, rel_base, 2, mode2)?;
                if a == 0 {
                    ip = b as usize;
                } else {
                    ip += 3;
                }
            }
            OP_LT => {
                let a = get_param(&mut mem, ip, rel_base, 1, mode1)?;
                let b = get_param(&mut mem, ip, rel_base, 2, mode2)?;
                let tgt = mem[ip + 3] as usize;
                mem[tgt] = (a < b) as Cell;
                ip += 4;
            }
            OP_EQ => {
                let a = get_param(&mut mem, ip, rel_base, 1, mode1)?;
                let b = get_param(&mut mem, ip, rel_base, 2, mode2)?;
                let tgt = mem[ip + 3] as usize;
                mem[tgt] = (a == b) as Cell;
                ip += 4;
            }
            OP_ARB => {
                let a = get_param(&mut mem, ip, rel_base, 1, mode1)?;
                if a < 0 {
                    rel_base -= a as usize;
                } else {
                    rel_base += a as usize;
                }
                println!("New rel_base={}", rel_base);
                ip += 2;
            }
            OP_HALT => {
                break;
            }
            _ => {
                return Err(MachineError::InvalidOpCode);
            }
        }
    }
    Ok(mem.v)
}

pub fn parse_mem(rd: &mut dyn BufRead) -> Vec<Cell> {
    rd.split(b',')
        .map(|op| {
            let op = op.unwrap();
            let op = std::str::from_utf8(&op).unwrap().trim();
            op.parse().unwrap()
        })
        .collect()
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

    #[test]
    fn test_output_eq() {
        for i in -100..100 {
            let mut vio = VecIO::new(vec![i]);
            execute(&mut vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], &mut vio).unwrap();
            assert_eq!(vio.output.len(), 1);
            assert_eq!(vio.output[0] == 1, i == 8);
            assert_eq!(vio.output[0] == 0, i != 8);
        }
    }

    #[test]
    fn test_output_le() {
        for i in -100..100 {
            let mut vio = VecIO::new(vec![i]);
            execute(&mut vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], &mut vio).unwrap();
            assert_eq!(vio.output.len(), 1);
            assert_eq!(vio.output[0] == 1, i < 8);
            assert_eq!(vio.output[0] == 0, i >= 8);
        }
    }

    #[test]
    fn test_output_eq_immediate() {
        for i in -100..100 {
            let mut vio = VecIO::new(vec![i]);
            execute(&mut vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], &mut vio).unwrap();
            assert_eq!(vio.output.len(), 1);
            assert_eq!(vio.output[0] == 1, i == 8);
            assert_eq!(vio.output[0] == 0, i != 8);
        }
    }

    #[test]
    fn test_output_le_immediate() {
        for i in -100..100 {
            let mut vio = VecIO::new(vec![i]);
            execute(&mut vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], &mut vio).unwrap();
            assert_eq!(vio.output.len(), 1);
            assert_eq!(vio.output[0] == 1, i < 8);
            assert_eq!(vio.output[0] == 0, i >= 8);
        }
    }

    #[test]
    fn test_j() {
        for i in -100..100 {
            let mut vio = VecIO::new(vec![i]);
            execute(
                &mut vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                &mut vio,
            )
            .unwrap();
            assert_eq!(vio.output.len(), 1);
            assert_eq!(vio.output[0] == 1, i != 0);
            assert_eq!(vio.output[0] == 0, i == 0);
        }
    }

    #[test]
    fn test_j_immediate() {
        for i in -100..100 {
            let mut vio = VecIO::new(vec![i]);
            execute(
                &mut vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
                &mut vio,
            )
            .unwrap();
            assert_eq!(vio.output.len(), 1);
            assert_eq!(vio.output[0] == 1, i != 0);
            assert_eq!(vio.output[0] == 0, i == 0);
        }
    }

    #[test]
    fn test_j_larger() {
        for i in -100..100 {
            let mut vio = VecIO::new(vec![i]);
            execute(
                &mut vec![
                    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
                    36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46,
                    1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
                ],
                &mut vio,
            )
            .unwrap();
            assert_eq!(vio.output.len(), 1);
            assert_eq!(vio.output[0] == 999, i < 8);
            assert_eq!(vio.output[0] == 1000, i == 8);
            assert_eq!(vio.output[0] == 1001, i > 8);
        }
    }

    #[test]
    fn test_arb() {
        assert_eq!(
            execute(&mut vec![109, 7, 1201, 0, 0, 0, 99, -1337], &mut EmptyIO {}).unwrap()[0],
            -1337
        );
    }

    #[test]
    fn test_quine() {
        let mut prog = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let orig_prog = prog.clone();
        let mut vio = VecIO::new(vec![]);
        execute(&mut prog, &mut vio).unwrap();
        assert_eq!(orig_prog, vio.output);
    }

    #[test]
    fn test_large_num() {
        let n = 1125899906842624;
        let mut prog = vec![104, n, 99];
        let mut vio = VecIO::new(vec![]);
        execute(&mut prog, &mut vio).unwrap();
        assert_eq!(vec![n], vio.output);
    }

    #[test]
    fn test_large_num2() {
        let a = 34915192;
        let b = 34915192;
        let mut prog = vec![1102, a, b, 7, 4, 7, 99, 0];
        let mut vio = VecIO::new(vec![]);
        execute(&mut prog, &mut vio).unwrap();
        assert_eq!(vec![a * b], vio.output);
    }
}
