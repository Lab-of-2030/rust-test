// 使用多线程并行计算某个函数的值或模拟并发任务。
// 需要创建 3 个线程同时进行下载，并在下载完成后将结果（例如“URL + 下载完成”）
// 通过消息通道（std::sync::mpsc）发送回主线程。主线程依次接收并打印结果。

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// Simulate a download function
fn download(url: &str) -> String {
    thread::sleep(Duration::from_secs(1)); // Simulate time taken for download
    format!("{} + 下载完成", url)
}

fn parallel_download(urls: Vec<&str>) -> Vec<String> {
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];

    for url in urls {
        let tx_clone = tx.clone();
        let url = url.to_string();
        let handle = thread::spawn(move || {
            let result = download(&url);
            tx_clone.send(result).unwrap();
        });
        handles.push(handle);
    }

    drop(tx); // Close the sending side

    let mut results = vec![];
    for received in rx {
        results.push(received);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    results
}

pub fn run() {
    let urls = vec![
        "http://example1.com",
        "http://example2.com",
        "http://example3.com",
    ];

    println!("开始并行下载...");
    let results = parallel_download(urls);

    println!("下载结果：");
    for result in results {
        println!("{}", result);
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download() {
        let url = "http://example.com";
        let result = download(url);
        assert_eq!(result, "http://example.com + 下载完成");
    }

    #[test]
    fn test_parallel_download() {
        let urls = vec!["http://example1.com", "http://example2.com", "http://example3.com"];
        let results = parallel_download(urls.clone());
        for url in urls {
            assert!(results.contains(&format!("{} + 下载完成", url)));
        }
    }

    #[test]
    fn test_parallel_download_empty() {
        let urls: Vec<&str> = vec![];
        let results = parallel_download(urls);
        assert!(results.is_empty());
    }

    #[test]
    fn test_parallel_download_order_independence() {
        let urls = vec!["http://example1.com", "http://example2.com", "http://example3.com"];
        let results = parallel_download(urls.clone());
        assert_eq!(results.len(), urls.len());
        for url in urls {
            assert!(results.contains(&format!("{} + 下载完成", url)));
        }
    }
}
