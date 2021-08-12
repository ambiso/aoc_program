use aoc_program::{parse_mem, execute};

fn main() {
	let mut mem = parse_mem();
	execute(&mut mem).unwrap();
	println!("mem[0]={}", mem[0]);
}
