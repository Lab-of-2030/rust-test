// 请从命令行读取一行字符串（例如 "apple banana pear banana apple banana"）。
// 使用空格进行拆分，统计每个单词出现的次数，并以从高到底的顺序输出。
// 如果出现次数相同，按单词本身的字典序从小到大排序输出。

use std::collections::HashMap;
use std::io::{self, Write};

pub fn count_words(input: &str) -> Vec<(String, usize)> {
    let mut word_count = HashMap::new();

    for word in input.split_whitespace() {
        *word_count.entry(word.to_string()).or_insert(0) += 1;
    }

    let mut word_count_vec: Vec<(String, usize)> = word_count.into_iter().collect();

    word_count_vec.sort_by(|a, b| {
        b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0))
    });

    word_count_vec
}

pub fn run() {
    // 提示用户输入
    print!("请输入一行字符串: ");
    io::stdout().flush().unwrap();

    // 从标准输入读取一行
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // 去掉输入末尾的换行符
    let input = input.trim();

    // 调用 count_words 函数
    let result = count_words(input);

    // 输出结果
    for (word, count) in result {
        println!("{}: {}", word, count);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_words_basic() {
        let input = "apple banana pear banana apple banana";
        let result = count_words(input);
        let expected = vec![
            ("banana".to_string(), 3),
            ("apple".to_string(), 2),
            ("pear".to_string(), 1),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_words_empty() {
        let input = "";
        let result = count_words(input);
        let expected: Vec<(String, usize)> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_words_single_word() {
        let input = "apple";
        let result = count_words(input);
        let expected = vec![("apple".to_string(), 1)];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_words_same_frequency() {
        let input = "apple banana apple banana";
        let result = count_words(input);
        let expected = vec![
            ("apple".to_string(), 2),
            ("banana".to_string(), 2),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_words_with_punctuation() {
        let input = "apple, banana. apple banana!";
        let result = count_words(input);
        let expected = vec![
            ("apple".to_string(), 1),
            ("apple,".to_string(), 1),
            ("banana!".to_string(), 1),
            ("banana.".to_string(), 1),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_words_case_sensitivity() {
        let input = "Apple apple APPLE";
        let result = count_words(input);
        let mut expected = vec![
            ("Apple".to_string(), 1),
            ("apple".to_string(), 1),
            ("APPLE".to_string(), 1),
        ];
        expected.sort();
        assert_eq!(result, expected);
    }
}
