// 实现一个命令行工具，对指定目录下的所有文本文件进行搜索，将匹配结果汇总后输出。
// 为增强可玩性和综合性，该工具需要支持：
// - 命令行参数（接收搜索关键词、目录路径、是否忽略大小写等）。
// - 并发搜索。
// - 消息通信。
// - 数据结构。
// - 错误处理。
// - 文件操作。
// - 迭代器与泛型（文本行迭代、搜索函数可考虑使用泛型或 trait 做一定延伸）。
// - 可选扩展：忽略大小写、正则匹配、统计行数或文件数等。

use std::io::{self, BufRead};
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::fs::{self, File};
use std::io::Write;
use tempfile::tempdir;
use std::env;

pub fn search_in_file(file_path: &Path, keyword: &str, case_insensitive: bool) -> io::Result<Vec<String>> {
    println!("Searching in file: {}", file_path.display());
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut results = Vec::new();
    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;
        let search_line = if case_insensitive {
            line.to_lowercase()
        } else {
            line.clone()
        };
        let search_keyword = if case_insensitive {
            keyword.to_lowercase()
        } else {
            keyword.to_string()
        };

        if search_line.contains(&search_keyword) {
            results.push(format!("{}: {}", line_number + 1, line));
        }
    }
    Ok(results)
}

pub fn search_in_directory(
    dir_path: &Path,
    keyword: &str,
    case_insensitive: bool,
) -> io::Result<Vec<String>> {
    let mut results = Vec::new();
    let (tx, rx) = mpsc::channel();

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "txt") {
            let tx = tx.clone();
            let keyword = keyword.to_string();
            thread::spawn(move || {
                if let Ok(matches) = search_in_file(&path, &keyword, case_insensitive) {
                    for match_line in matches {
                        tx.send(format!("{}: {}", path.display(), match_line)).unwrap();
                    }
                }
            });
        }
    }

    drop(tx); // Close the sender to avoid deadlock
    for received in rx {
        results.push(received);
    }

    Ok(results)
}

pub fn run(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 4 {
        eprintln!("Usage: <program> <directory> <keyword> [--case-insensitive]");
        return Err("Insufficient arguments".into());
    }

    let dir_path = Path::new(&args[2]);
    let keyword = &args[3];
    let case_insensitive = args.iter().any(|arg| arg == "--case-insensitive");

    eprint!("Searching for '{}' in directory '{}'\n", keyword, dir_path.display());

    if !dir_path.is_dir() {
        eprintln!("Error: {} is not a valid directory", dir_path.display());
        return Err("Invalid directory".into());
    }

    let results = search_in_directory(dir_path, keyword, case_insensitive)?;

    if results.is_empty() {
        println!("No matches found.");
    } else {
        for result in results {
            println!("{}", result);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_in_file_case_sensitive() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Hello World").unwrap();
        writeln!(file, "Rust programming").unwrap();

        let results = search_in_file(&file_path, "World", false).unwrap();
        assert_eq!(results, vec!["1: Hello World"]);
    }

    #[test]
    fn test_search_in_file_case_insensitive() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Hello World").unwrap();
        writeln!(file, "Rust programming").unwrap();

        let results = search_in_file(&file_path, "world", true).unwrap();
        assert_eq!(results, vec!["1: Hello World"]);
    }

    #[test]
    fn test_search_in_directory() {
        let dir = tempdir().unwrap();
        let file1_path = dir.path().join("test1.txt");
        let file2_path = dir.path().join("test2.txt");

        let mut file1 = File::create(&file1_path).unwrap();
        writeln!(file1, "Hello World").unwrap();

        let mut file2 = File::create(&file2_path).unwrap();
        writeln!(file2, "Rust programming").unwrap();

        let results = search_in_directory(dir.path(), "Hello", false).unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].contains("test1.txt"));
        assert!(results[0].contains("Hello World"));
    }

    #[test]
    fn test_search_in_directory_case_insensitive() {
        let dir = tempdir().unwrap();
        let file1_path = dir.path().join("test1.txt");
        let file2_path = dir.path().join("test2.txt");

        let mut file1 = File::create(&file1_path).unwrap();
        writeln!(file1, "Hello World").unwrap();

        let mut file2 = File::create(&file2_path).unwrap();
        writeln!(file2, "Rust programming").unwrap();

        let results = search_in_directory(dir.path(), "hello", true).unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].contains("test1.txt"));
        assert!(results[0].contains("Hello World"));
    }

    #[test]
    fn test_search_in_empty_directory() {
        let dir = tempdir().unwrap();
        let results = search_in_directory(dir.path(), "Hello", false).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_search_in_file_not_found() {
        let result = search_in_file(Path::new("non_existent.txt"), "Hello", false);
        assert!(result.is_err());
    }
}
