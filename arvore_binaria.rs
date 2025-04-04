struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

struct BinaryTree {
    root: Option<Box<TreeNode>>,
}

impl BinaryTree {
    fn new() -> Self {
        BinaryTree { root: None }
    }
    
    fn insert(&mut self, value: i32) {
        let new_node = Box::new(TreeNode { value, left: None, right: None });
        self.root = Some(new_node);
    }
}

fn main() {
    let mut tree = BinaryTree::new();
    tree.insert(42);
    println!("Árvore criada com raiz 42!");
}
