use std::thread;
use std::time::Duration;
use std::time::Instant;
use tokio::time::sleep;
#[tokio::main]
async fn main() {
    let start_time = Instant::now();
    let coffee_mug_thread = tokio::spawn(async {
        prep_coffee_mug().await;
    });
    let coffee_thread = tokio::spawn(async {
        make_coffee().await;
    });
    let toast_thread = tokio::spawn(async {
        make_toast().await;
    });

    tokio::join!(coffee_mug_thread, coffee_thread, toast_thread);

    let elapsed_time = start_time.elapsed();
    println!("It took: {} seconds", elapsed_time.as_secs());
}

async fn prep_coffee_mug() {
    let thread_name = thread::current().name().unwrap_or("Unnamed").to_string();
    println!("Current thread name: {}", thread_name);
    sleep(Duration::from_secs(1)).await;
    println!("Pouring milk...blocking");
    thread::sleep(Duration::from_secs(3));
    println!("Milk poured.");
    println!("Putting instant coffee...blocking");
    thread::sleep(Duration::from_secs(3));
    println!("Instant coffee put.");
}

async fn make_coffee() {
    let thread_name = thread::current().name().unwrap_or("Unnamed").to_string();
    println!("Current thread name: {}", thread_name);
    println!("boiling kettle...switching");
    sleep(Duration::from_secs(10)).await;
    println!("kettle boiled.");
    println!("pouring boiled water...blocking");
    thread::sleep(Duration::from_secs(3));
    println!("boiled water poured.");
}

async fn make_toast() {
    let thread_name = thread::current().name().unwrap_or("Unnamed").to_string();
    println!("Current thread name: {}", thread_name);
    println!("putting bread in toaster...switching");
    sleep(Duration::from_secs(10)).await;
    println!("bread toasted.");
    println!("buttering toasted bread...blocking");
    thread::sleep(Duration::from_secs(5));
    println!("toasted bread buttered.");
}
