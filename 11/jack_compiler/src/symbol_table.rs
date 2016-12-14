use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum IdentifierKind {
    STATIC,
    FIELD,
    ARG,
    VAR
}

struct Record {     
    var_type: String,
    kind: IdentifierKind, 
    index: u16
}   

struct Table {
    records: HashMap<String, Record>,
    counts: HashMap<IdentifierKind, u16>
}

impl Table {
    pub fn new() -> Self {
        let mut counts = HashMap::new();
        counts.insert(IdentifierKind::STATIC, 0);
        counts.insert(IdentifierKind::FIELD, 0);
        counts.insert(IdentifierKind::ARG, 0);
        counts.insert(IdentifierKind::VAR, 0);

        Table {
            records: HashMap::new(),
            counts: counts
        }
    }
}

pub struct SymbolTable {
    class_table: Table,
    subroutine_table: Table
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            class_table: Table::new(),
            subroutine_table: Table::new()
        }
    }

    pub fn start_subroutine(&mut self) {
        self.subroutine_table = Table::new()
    }

    pub fn define(&mut self, name: String, var_type: String, kind: IdentifierKind) {
        unimplemented!()
    }

    pub fn var_count(&self, kind: IdentifierKind) -> u16 {
        unimplemented!()
    }

    pub fn kind_of(&self, name: String) -> IdentifierKind {
        unimplemented!()
    }

    pub fn type_of(&self, name: String) -> String {
        unimplemented!()
    }

    pub fn index_of(&self, name: String) -> String {
        unimplemented!()
    }
}
