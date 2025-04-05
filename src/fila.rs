struct Queue {
    elements: Vec<i32>,
}

impl Queue {
    fn new() -> Self {
        Queue { elements: Vec::new() }
    }
    
    fn enqueue(&mut self, value: i32) {
        self.elements.push(value);
    }
    
    fn dequeue(&mut self) -> Option<i32> {
        if self.elements.is_empty() {
            None
        } else {
            Some(self.elements.remove(0))
        }
    }
}

fn main() {
    let mut queue = Queue::new();
    queue.enqueue(5);
    queue.enqueue(10);
    println!("Primeiro da fila: {:?}", queue.dequeue());
}
