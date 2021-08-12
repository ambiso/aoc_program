use anyhow::Result;
use aoc_program::{execute, parse_mem, LineIO, IO};
use std::{fs::File, io::BufReader, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc_program", about = "AoC interpreter")]
struct Opt {
    /// Program file
    #[structopt(parse(from_os_str))]
    program: PathBuf,

    /// Input file
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let mut mem = parse_mem(&mut BufReader::new(File::open(opt.program)?));
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut io: Box<dyn IO> = match opt.input {
        Some(input_path) => Box::new(LineIO::new(
            BufReader::new(File::open(&input_path)?),
            stdout.lock(),
        )),
        None => Box::new(LineIO::new(stdin.lock(), stdout.lock())),
    };
    execute(&mut mem, &mut io)?;
    println!("mem[0]={}", mem[0]);
    Ok(())
}
