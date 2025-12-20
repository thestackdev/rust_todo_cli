use std::io::{self, Write};

pub fn read(input: &mut String) {
    io::stdin().read_line(input).expect("Failed to read input");
}

pub fn flush_output() {
    io::stdout().flush().expect("Failed to flush output");
}
