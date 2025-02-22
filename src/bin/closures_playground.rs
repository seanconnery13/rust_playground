use std::thread;

fn main() {
    println!("Borrow example");
    borrow_example();
    println!("Move example");
    move_example();
}

fn borrow_example() {
    // borrow mutably, as the operation of push
    // is mutating the list
    let mut list = vec![1, 2, 3];
    let mut closure = || {
        list.push(4);
    };
    // cannot have this println statement as no immutable borrow post mutable borrow
    // println!("{:?}", list);
    closure();
    println!("{:?}", list);
}

fn move_example() {
    let list = vec![1, 2, 3];
    // unwrap call is needed as join() returns a Result which could be an error
    // and we want to unwrap that
    thread::spawn(move || {
        println!("{:?}", list);
    })
    .join()
    .unwrap();

    //below does not as value has moved
    //println!("{:?}", list);
}
