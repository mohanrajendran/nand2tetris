use std::collections::HashMap;

pub struct SymbolTable<'a> {
    hash_map: HashMap<&'a str, u16>
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> Self {
        let mut hash_map = HashMap::new();

        hash_map.insert("SP", 0);
        hash_map.insert("LCL", 1);
        hash_map.insert("ARG", 2);
        hash_map.insert("THIS", 3);
        hash_map.insert("THAT", 4);
        hash_map.insert("R0", 0);
        hash_map.insert("R1", 1);
        hash_map.insert("R2", 2);
        hash_map.insert("R3", 3);
        hash_map.insert("R4", 4);
        hash_map.insert("R5", 5);
        hash_map.insert("R6", 6);
        hash_map.insert("R7", 7);
        hash_map.insert("R8", 8);
        hash_map.insert("R9", 9);
        hash_map.insert("R10", 10);
        hash_map.insert("R11", 11);
        hash_map.insert("R12", 12);
        hash_map.insert("R13", 13);
        hash_map.insert("R14", 14);
        hash_map.insert("R15", 15);
        hash_map.insert("SCREEN", 16384);
        hash_map.insert("KBD", 24576);

        SymbolTable {
            hash_map: hash_map
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
