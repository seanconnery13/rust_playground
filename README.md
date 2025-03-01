# rust_playground
playground for RUST 🦀

### Notes
* Rust's ownership rules are soo much like pessimistic locking !
  Creating a mutable reference is like grabbing hold of the lock (to the memory). 
  Post that no references (either mutable or immutable) can be created until the lifetime of the first mutable reference.
* Calling join on the handle blocks the thread currently running until the thread represented by the handle terminates 
* Deref coercion is an automatic reference to reference conversion. By implementing the deref trait on a type, you are defining behavior of how to dereference.
  * Rust will then automatically dereference a type - when there is mismatch in a function
  * test_deref_coercion(&mybox); fn test_deref_coercion(x : &String)
  * dereference of mybox returns a reference to a string. So Rust automatically does that for you !
* also auto-dereferencing of references in rust happens when you use the dot operator with the reference (function call, data access)
  * lets say reference r points to a struct. That struct has a function foo. There is no need to do *r.foo. r.foo works. foo can also be data access
