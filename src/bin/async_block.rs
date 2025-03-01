#[tokio::main]
async fn main() {
    let fut1 = async {
       println!("first async block");
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    };
    let fut2 = async {
        println!("second async block");
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    };
    fut2.await;
    //there is no multithreading here. Simply showcasing usage of Future.
    fut1.await;
}