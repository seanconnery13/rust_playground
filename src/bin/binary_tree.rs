use std::rc::Rc;

fn main() {
    let left = Rc::new(Node {
        left: None,
        right: None,
        data: 1,
    });

    let right = Rc::new(Node {
        left: None,
        right: None,
        data: 3,
    });

    let head_node = Some(Rc::new(Node {
        left: Some(Rc::clone(&left)),
        right: Some(Rc::clone(&right)),
        data: 2,
    }));

    traverse(head_node);
}

fn traverse(n: Option<Rc<Node>>) {
    if let Some(rcnode) = n {
        println!("{}", rcnode.data);
        if let Some(ref left) = rcnode.left {
            traverse(Some(Rc::clone(left)));
        }
        if let Some(ref right) = rcnode.right {
            traverse(Some(Rc::clone(right)));
        }
    }
}

struct Node {
    left: Option<Rc<Node>>,
    right: Option<Rc<Node>>,
    data: u8,
}
