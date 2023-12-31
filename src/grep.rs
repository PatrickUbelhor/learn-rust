use std::error::Error;
use clap::Parser;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::fs::File;

#[derive(Parser)]
struct Cli {
	pattern: String,
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
	let stdout = io::stdout(); // Get the global stdout entity
	let handle = io::BufWriter::new(stdout); // Wrap the handler in a buffer

	find_matches(&args.pattern, br, handle)?;

	Ok(())
}

fn find_matches(
	pattern: &str,
	mut reader: BufReader<File>,
	mut writer: impl Write
) -> Result<(), Box<dyn Error>> {

	let mut line = String::new();
	while reader.read_line(&mut line).unwrap() > 0 {
		if line.contains(pattern) {
			writeln!(writer, "{}", line)?;
			line.clear();
		}
	}

	writer.flush()?;

	Ok(())
}

#[test]
fn test_find_matches() {
	let mut result = Vec::new();
	find_matches("This is a test\nanother line\nThird is last", "is", &mut result);
	assert_eq!(result, b"This is a test\nThird is last\n");
}
