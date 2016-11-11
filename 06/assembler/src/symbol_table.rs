use std::collections::HashMap;

pub struct SymbolTable<'a> {
    hash_map: HashMap<&'a str, u16>
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> Self {
        SymbolTable {
            hash_map: HashMap::new()
        }
    }

    pub fn add_entry(&mut self, symbol: &'a str, address: u16) -> () {
        self.hash_map.insert(symbol, address);
    }

    pub fn contains(&self, symbol: &str) -> bool {
        self.hash_map.contains_key(symbol)
    }

    pub fn get_address(&self, symbol: &str) -> u16 {
        self.hash_map.get(symbol).expect("Unable to find symbol").clone()
    }
}
