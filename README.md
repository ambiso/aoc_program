# Advent of Code 2019

I'm only doing the Intcode computer challenges here.

## Day 2

```bash
$ cargo run --bin aoc_program -- program_day2.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/aoc_program`
mem[0]=3760627
```

```bash
$ cargo run --bin inputfinder -- program_day2.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/inputfinder`
Noun=71 Verb=95; 7195
```

## Day 5

```bash
echo 1 | cargo run --bin aoc_program -- program_day5.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/aoc_program program_day5.txt`
0
0
0
0
0
0
0
0
0
12896948
mem[0]=3
```

```bash
echo 5 |cargo run --bin aoc_program -- program_day5.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/aoc_program program_day5.txt`
7704130
mem[0]=314
```

## Day 9

```bash
echo 1 | time cargo run --release --bin aoc_program -- program_day9.txt
    Finished release [optimized] target(s) in 0.02s
     Running `target/release/aoc_program program_day9.txt`
2171728567
mem[0]=1102
cargo run --release --bin aoc_program -- program_day9.txt  0.11s user 0.02s system 99% cpu 0.125 total
```

```bash
echo 2 | time cargo run --release --bin aoc_program -- program_day9.txt
    Finished release [optimized] target(s) in 0.02s
     Running `target/release/aoc_program program_day9.txt`
49815
mem[0]=1102
cargo run --release --bin aoc_program -- program_day9.txt  0.11s user 0.01s system 99% cpu 0.120 total
```