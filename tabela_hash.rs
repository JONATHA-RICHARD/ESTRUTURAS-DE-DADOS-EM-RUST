use std::collections::HashMap;

struct HashTable {
    table: HashMap<String, i32>,
}

impl HashTable {
    fn new() -> Self {
        HashTable { table: HashMap::new() }
    }
    
    fn insert(&mut self, key: String, value: i32) {
        self.table.insert(key, value);
    }
    
    fn get(&self, key: &String) -> Option<&i32> {
        self.table.get(key)
    }
}

fn main() {
    let mut ht = HashTable::new();
    ht.insert("chave1".to_string(), 100);
    println!("Valor associado: {:?}", ht.get(&"chave1".to_string()));
}
