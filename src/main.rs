use std::error::Error;
use clap::Parser;
use std::io::{self, BufReader, BufWriter, Read, Stdout, Write};
use std::fs::File;

#[derive(Parser)]
struct Cli {
	path: std::path::PathBuf
}

const SIZE_BUFFERED_READER: usize = 8 * 1024; // Buffer size of reader, in bytes
const SIZE_BUFFERED_WRITER: usize = 8 * 1024; // Buffer size of writer, in bytes

fn main() -> Result<(), Box<dyn Error>> {
	let args = Cli::parse();

	let file: File = match File::open(&args.path) {
		Ok(file) => file,
		Err(error) => { panic!("Unable to open file | {}", error); }
	};
	let br: BufReader<File> = BufReader::with_capacity(SIZE_BUFFERED_READER, file);
	let stdout: Stdout = io::stdout(); // Get the global stdout entity
	let handle: BufWriter<Stdout> = BufWriter::with_capacity(SIZE_BUFFERED_WRITER, stdout); // Wrap the handler in a buffer

	print_file(br, handle)?;

	Ok(())
}

fn print_file(
	mut reader: BufReader<File>,
	mut writer: impl Write
) -> Result<(), Box<dyn Error>> {

	let mut buffer: [u8; SIZE_BUFFERED_READER] = [0; SIZE_BUFFERED_READER];
	while reader.read(&mut buffer).unwrap() > 0 {
		write!(writer, "{:?}", buffer)?;
		buffer = [0; SIZE_BUFFERED_READER];
	}

	writer.flush()?;

	Ok(())
}
