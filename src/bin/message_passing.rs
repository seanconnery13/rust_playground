use std::sync::mpsc;
use std::thread;

fn main() {
    // multiple producer and single consumer
    // tx = sender = transmitter
    // rx = receiver

    // we will have 4 senders
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    let tx3 = tx.clone();
    let tx4 = tx.clone();

    // spawned thread owns tx
    // send operation returns a Result - so we call unwrap
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // this is not possible as send takes ownership
        // recv owns this value
        //println!("tx val: {}", val);
    });
    thread::spawn(move || {
        let val2 = String::from("my");
        tx2.send(val2).unwrap();
    });
    thread::spawn(move || {
        let val3 = String::from("name is");
        tx3.send(val3).unwrap();
    });
    thread::spawn(move || {
        let val4 = String::from("Dhaval Shownkani");
        tx4.send(val4).unwrap();
    });

    for received in rx {
        println!("Got: {}", received);
    }
}