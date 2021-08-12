use aoc_program::{execute, parse_mem, LineIO};

fn main() {
    let mut mem = parse_mem();
    execute(&mut mem, &mut LineIO {}).unwrap();
    println!("mem[0]={}", mem[0]);
}
