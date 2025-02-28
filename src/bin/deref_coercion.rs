use std::ops::Deref;

fn main() {
    // deref coercion
    //so by implementing the deref trait
    // we have added automatic conversion from &MyBox<T> to &T ? Yes
    let mybox = MyBox::new(String::from("deref coercion works"));
    test_deref_coercion(&mybox);
}

fn test_deref_coercion(x: &String) {
    println!("{}", x);
}

// struct that stores T
struct MyBox<T>(T);

//adding behavior
// add a new function that creates MyBox
impl<T> MyBox<T> {
    fn new(input: T) -> MyBox<T> {
        MyBox(input)
    }
}

// implement deref trait for MyBox
impl<T> Deref for MyBox<T> {
    // associated property
    type Target = T;
    // takes a immutable reference to self
    // and returns type T
    // notice Self vs self
    // Self refers to the type and self refers to an instance of that type
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
