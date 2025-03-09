use std::thread;
use std::time::Duration;
use std::time::Instant;
use tokio::time::sleep;
#[tokio::main]
async fn main() {
    let start_time = Instant::now();
    let coffee_mug_step = prep_coffee_mug();
    let coffee_step = make_coffee();
    let toast_step = make_toast();

    tokio::join!(coffee_mug_step, coffee_step, toast_step);
    // if await was called on tasks early, then its basically sync code running. join! helps you run these tasks concurrently on the same thread.
    // prep_coffee_mug().await;
    // make_coffee().await;
    // make_toast().await;

    let elapsed_time = start_time.elapsed();
    println!("It took: {} seconds", elapsed_time.as_secs());
}
async fn prep_coffee_mug() {
    println!("Pouring milk...blocking");
    thread::sleep(Duration::from_secs(3));
    println!("Milk poured.");
    println!("Putting instant coffee...blocking");
    thread::sleep(Duration::from_secs(3));
    println!("Instant coffee put.");
}

async fn make_coffee() {
    println!("boiling kettle...switching");
    sleep(Duration::from_secs(10)).await;
    println!("kettle boiled.");
    println!("pouring boiled water...blocking");
    thread::sleep(Duration::from_secs(3));
    println!("boiled water poured.");
}

async fn make_toast() {
    println!("putting bread in toaster...switching");
    sleep(Duration::from_secs(10)).await;
    println!("bread toasted.");
    println!("buttering toasted bread...blocking");
    thread::sleep(Duration::from_secs(5));
    println!("toasted bread buttered.");
}
