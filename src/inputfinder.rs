use aoc_program::{parse_mem, execute};

fn main() {
	let mem = parse_mem();
	let target = 19690720;
	for noun in 0..100 {
		for verb in 0..100 {
			let mut mem2 = mem.clone();
			mem2[1] = noun;
			mem2[2] = verb;
			execute(&mut mem2).unwrap();
			if mem2[0] == target {
				println!("Noun={} Verb={}; {}", noun, verb, 100*noun + verb);
				return;
			}
		}
	}
}