use std::thread;
use std::time::Duration;
use std::time::Instant;
use tokio::time::sleep;

// in the breakfast example, as the coffee_mug step is added first to the join! macro.
// it starts executing first and that is not the most efficient order
// we would rather want boiling water in the kettle and putting bread in the toaster to be started first
// as these activities are progressing, we can switch to other tasks - making the entire system more efficient
// to do so we don't change the order of tasks in the join macro but instead add a non-blocking sleep to prep_coffee_mug
// this way we can immediately switch boiling water and toasting bread. This is called cooperative multitasking
#[tokio::main]
async fn main() {
    let start_time = Instant::now();
    let coffee_mug_step = prep_coffee_mug();
    let coffee_step = make_coffee();
    let toast_step = make_toast();

    tokio::join!(coffee_mug_step, coffee_step, toast_step);

    let elapsed_time = start_time.elapsed();
    println!("It took: {} seconds", elapsed_time.as_secs());
}
async fn prep_coffee_mug() {
    sleep(Duration::from_secs(1)).await;
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
