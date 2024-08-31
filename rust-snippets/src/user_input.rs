use std::io::{self, BufRead, Write};

pub fn get_user_input<R: BufRead, W: Write>(reader: &mut R, writer: &mut W) {
    let mut input = String::new();
    reader.read_line(&mut input).expect("failed to read");
    let input = input.trim().to_string();
    writeln!(writer, "Entered:").unwrap();
    writeln!(writer, "{}", input).unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_get_user_input() {
        // Simulate user input with Cursor
        let input_data = b"Hello, world!\n";  // Simulated input
        let mut reader = Cursor::new(input_data);
        
        // Capture output with Vec
        let mut output = Vec::new();

        // Use the refactored function
        get_user_input(&mut reader, &mut output);

        // Convert the output to a String and split by lines
        let output_str = String::from_utf8(output).unwrap();
        let output_lines: Vec<&str> = output_str.lines().collect();

        // Check if the output matches expected output
        assert_eq!(output_lines[0], "Entered:");
        assert_eq!(output_lines[1], "Hello, world!");
    }
}
