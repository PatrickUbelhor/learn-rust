mod grep;

use std::error::Error;
use clap::Parser;
use std::io::{self, Write};

#[derive(Parser)]
struct Cli {
	pattern: String,
	path: std::path::PathBuf
}

// TODO: Use BufReader instead of loading the whole file into memory
fn main() -> Result<(), Box<dyn Error>> {
	let args = Cli::parse();
	let result = std::fs::read_to_string(&args.path);

	let content = match result {
		Ok(content) => { content },
		Err(error) => { panic!("Unable to open file | {}", error); }
	};


	let stdout = io::stdout(); // Get the global stdout entity
	let handle = io::BufWriter::new(stdout); // Wrap the handler in a buffer
	find_matches(&content, &args.pattern, handle)?;

	Ok(())
}

fn find_matches(content: &str, pattern: &str, mut writer: impl Write) -> Result<(), Box<dyn Error>> {
	for line in content.lines() {
		if line.contains(pattern) {
			writeln!(writer, "{}", line)?; // TODO: Add ? to handle errors here
		}
	}
	writer.flush()?; // TODO: Add ? to handle errors here

	Ok(())
}

#[test]
fn test_find_matches() {
	let mut result = Vec::new();
	find_matches("This is a test\nanother line\nThird is last", "is", &mut result);
	assert_eq!(result, b"This is a test\nThird is last\n");
}
