pub type Mem = Vec<u32>;
use std::io::BufRead;

pub fn execute(mem: &mut Mem) -> Result<&mut Mem, String> {
    let mut ip = 0;
    loop {
        let op = mem[ip];
        match op {
            1 => {
                let a = mem[mem[ip + 1] as usize];
                let b = mem[mem[ip + 2] as usize];
                let tgt = mem[ip + 3] as usize;
                mem[tgt] = a + b;
                ip += 4;
            }
            2 => {
                let a = mem[mem[ip + 1] as usize];
                let b = mem[mem[ip + 2] as usize];
                let tgt = mem[ip + 3] as usize;
                mem[tgt] = a * b;
                ip += 4;
            }
            99 => {
                break;
            }
            _ => {
                return Err(format!("Invalid opcode {} @ {}", op, ip));
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
            execute(&mut vec![1, 0, 0, 0, 99]),
            Ok(&mut vec![2, 0, 0, 0, 99])
        );
    }

    #[test]
    fn test_mul() {
        assert_eq!(
            execute(&mut vec![2, 3, 0, 3, 99]),
            Ok(&mut vec![2, 3, 0, 6, 99])
        );
    }

    #[test]
    fn test_mul2() {
        assert_eq!(
            execute(&mut vec![2, 4, 4, 5, 99, 0]),
            Ok(&mut vec![2, 4, 4, 5, 99, 9801])
        );
    }

    #[test]
    fn test_add2() {
        assert_eq!(
            execute(&mut vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            Ok(&mut vec![30, 1, 1, 4, 2, 5, 6, 0, 99])
        );
    }
}
