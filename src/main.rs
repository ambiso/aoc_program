use anyhow::Result;
use aoc_program::{execute, parse_mem, LineIO, IO};
use std::{fs::File, io::BufReader, path::PathBuf};
use structopt::StructOpt;
use tracing::Level;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc_program", about = "AoC interpreter")]
struct Opt {
    /// Program file
    #[structopt(parse(from_os_str))]
    program: PathBuf,

    /// Input file
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,

	#[structopt(short, long)]
	verbose: bool,
}

fn main() -> Result<()> {
	let opt = Opt::from_args();
	let subscriber = tracing_subscriber::FmtSubscriber::builder()
			// all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
			// will be written to stdout.
			.with_max_level(if opt.verbose {
				Level::TRACE
			} else {
				Level::INFO
			})
			// builds the subscriber.
			.finish();

    tracing::subscriber::with_default(subscriber, || {
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
    })
}
