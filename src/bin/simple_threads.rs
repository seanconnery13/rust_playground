use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

fn main() {
    //create_new_threads_without_join();
    let handler = create_new_threads_with_join();
    for i in 1..50 {
        println!("main thread");
        // need to sleep the main thread for
        thread::sleep(Duration::from_millis(1));
    }
    // Calling join on the handle blocks the thread currently running until the thread represented by the handle terminates
    // where you call join is important
    // if you were to call join at line 8, then the main thread would be blocked until the spawned thread completes
    handler.join().unwrap();
}

fn create_new_threads_without_join() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
}

// JoinHandle belongs to a thread
// calling join method on the join handle,
// will ensure this thread completes execution
fn create_new_threads_with_join() -> JoinHandle<()> {
    let handler = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handler
}
