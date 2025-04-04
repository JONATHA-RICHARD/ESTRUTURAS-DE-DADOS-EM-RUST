struct Stack {
    elements: Vec<i32>,
}

impl Stack {
    fn new() -> Self {
        Stack { elements: Vec::new() }
    }
    
    fn push(&mut self, value: i32) {
        self.elements.push(value);
    }
    
    fn pop(&mut self) -> Option<i32> {
        self.elements.pop()
    }
}

fn main() {
    let mut stack = Stack::new();
    stack.push(10);
    stack.push(20);
    println!("Topo da pilha: {:?}", stack.pop());
}
