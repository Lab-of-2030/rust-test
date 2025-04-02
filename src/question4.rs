// 从命令行参数接收一个文件路径，例如 input.txt。如果没有提供路径或文件无法打开，给出相应的错误提示并退出。
// 读取该文件的所有文本内容，统计文本中一共包含的字符数（不含换行符）与行数，并将结果写入 output.txt。
// 若 output.txt 文件已存在，可以选择直接覆盖或者追加，任选其一，但需要在程序里明确注释或说明处理方式。

use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

fn process_file(input_path: &str, output_path: &str) -> Result<(), String> {
    // Read the file content
    let content = fs::read_to_string(input_path).map_err(|_| "Failed to read input file")?;

    // Count characters (excluding newlines) and lines
    let char_count = content.chars().filter(|&c| c != '\n').count();
    let line_count = content.lines().count();

    // Prepare the result string
    let result = format!("Characters: {}\nLines: {}\n", char_count, line_count);

    // Write to output.txt (overwriting if it exists)
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true) // Overwrite the file
        .open(output_path)
        .map_err(|_| "Failed to write to output file")?;
    file.write_all(result.as_bytes())
        .map_err(|_| "Failed to write to output file")?;

    Ok(())
}

pub fn run(input_path: &str, output_path: &str) -> Result<(), String> {
    process_file(input_path, output_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_file_valid_input() {
        let input_path = "test_valid_input.txt";
        let output_path = "test_valid_output.txt";

        // Create a test input file
        fs::write(input_path, "Hello\nWorld\n").unwrap();

        // Process the file
        let result = process_file(input_path, output_path);
        assert!(result.is_ok());

        // Verify the output
        let output_content = fs::read_to_string(output_path).unwrap();
        assert_eq!(output_content, "Characters: 10\nLines: 2\n");

        // Clean up
        fs::remove_file(input_path).unwrap();
        fs::remove_file(output_path).unwrap();
    }

    #[test]
    fn test_process_file_missing_input() {
        let input_path = "non_existent.txt";
        let output_path = "test_output.txt";

        // Process the file
        let result = process_file(input_path, output_path);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Failed to read input file");
    }

    #[test]
    fn test_process_file_output_write_error() {
        let input_path = "test_input.txt";
        let output_path = "/invalid_path/output.txt";

        // Create a test input file
        fs::write(input_path, "Hello\nWorld\n").unwrap();

        // Process the file
        // Ensure the input file exists
        fs::write(input_path, "Hello\nWorld\n").unwrap();

        let result = process_file(input_path, output_path);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Failed to write to output file");

        // Clean up
        fs::remove_file(input_path).unwrap();
    }

    #[test]
    fn test_process_file_empty_input() {
        let input_path = "empty_input.txt";
        let output_path = "test_output.txt";

        // Create an empty test input file
        fs::write(input_path, "").unwrap();

        // Process the file
        let result = process_file(input_path, output_path);
        assert!(result.is_ok());

        // Verify the output
        let output_content = fs::read_to_string(output_path).unwrap();
        assert_eq!(output_content, "Characters: 0\nLines: 0\n");

        // Clean up
        fs::remove_file(input_path).unwrap();
        fs::remove_file(output_path).unwrap();
    }
}
