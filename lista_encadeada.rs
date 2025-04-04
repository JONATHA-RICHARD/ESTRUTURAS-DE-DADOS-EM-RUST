struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }
    
    fn push(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    println!("Lista criada com 3 elementos!");
}
