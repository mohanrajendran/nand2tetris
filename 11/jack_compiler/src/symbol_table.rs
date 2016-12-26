use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum IdentifierKind {
    STATIC,
    FIELD,
    ARG,
    VAR,
}

#[derive(Clone)]
struct Record {
    var_type: String,
    kind: IdentifierKind,
    index: u16,
}

impl Record {
    pub fn new(var_type: String, kind: IdentifierKind, index: u16) -> Record {
        Record {
            var_type: var_type,
            kind: kind,
            index: index,
        }
    }
}

struct Table {
    records: HashMap<String, Record>,
    counts: HashMap<IdentifierKind, u16>,
}

impl Table {
    pub fn new() -> Self {
        Table {
            records: HashMap::new(),
            counts: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, var_type: String, kind: IdentifierKind) {
        let index = self.counts.entry(kind).or_insert(0);

        let record = Record::new(var_type, kind, index.to_owned());
        self.records.insert(name, record);

        *index += 1;
    }

    pub fn get_count(&self, kind: IdentifierKind) -> u16 {
        match self.counts.get(&kind) {
            Some(v) => v.to_owned(),
            None => 0,
        }
    }

    pub fn get_record(&self, name: String) -> Option<Record> {
        self.records.get(&name).cloned()
    }
}

pub struct SymbolTable {
    class_table: Table,
    subroutine_table: Table,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            class_table: Table::new(),
            subroutine_table: Table::new(),
        }
    }

    pub fn start_subroutine(&mut self) {
        self.subroutine_table = Table::new()
    }

    pub fn define(&mut self, name: String, var_type: String, kind: IdentifierKind) {
        match kind {
            IdentifierKind::STATIC | IdentifierKind::FIELD => {
                self.class_table.insert(name, var_type, kind);
            }
            _ => {
                self.subroutine_table.insert(name, var_type, kind);
            }
        }
    }

    pub fn var_count(&self, kind: IdentifierKind) -> u16 {
        self.class_table.get_count(kind) + self.subroutine_table.get_count(kind)
    }

    fn get_record(&self, name: String) -> Record {
        let record = match self.subroutine_table.get_record(name.clone()) {
            Some(r) => Some(r),
            None => self.class_table.get_record(name)
        };

        record.expect("Identifier not found!")
    }

    pub fn kind_of(&self, name: String) -> IdentifierKind {
        self.get_record(name).kind
    }

    pub fn type_of(&self, name: String) -> String {
        self.get_record(name).var_type
    }

    pub fn index_of(&self, name: String) -> u16 {
        self.get_record(name).index
    }
}
