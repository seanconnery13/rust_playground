use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        for i in 1..=10 {
            println!("first thread - {}", i);
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });
    tokio::spawn(async {
        for i in 1..=5 {
            println!("Second thread - {}", i);
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });
    thread::sleep(Duration::from_millis(10000));
}
