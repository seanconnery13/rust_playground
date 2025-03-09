use futures::future::join;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let f1 = async {
        for i in 1..10 {
            println!("hi number {i} from the first task!");
            sleep(Duration::from_millis(500)).await;
        }
    };

    let f2 = async {
        for i in 1..5 {
            println!("hi number {i} from the second task!");
            sleep(Duration::from_millis(500)).await;
        }
    }
    .await;

    f1.await;
}
