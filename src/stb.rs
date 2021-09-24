use std::{borrow::{Borrow, BorrowMut}, cell::{Ref, RefCell, RefMut}, collections::HashMap, num::NonZeroU32, os::unix::process::parent_id, rc::{Rc, Weak}};

use crate::ast::Class;
#[derive(Debug, Clone)]
pub enum SymbolAttributes {
    Identifier(Class),
    Function(Vec<(String, Class)>),
}
#[derive(Debug, Clone)]
pub struct Symbol {
    pub lexeme: String,
    pub attributes: SymbolAttributes,
}
#[derive(Debug, Clone)]
struct SymbolTable {
    parent: Option<SymbolTableId>,
    symbols: HashMap<String, Symbol>,
    scopes: Vec<SymbolTableId>,
}
#[derive(Debug, Default, Clone, Copy)]
struct SymbolTableId(usize);
#[derive(Debug, Clone)]
pub struct SymbolTableManager {
    current_table_id: SymbolTableId,
    tables: Vec<SymbolTable>
}

#[derive(Debug)]
pub enum Error {
    RedeclareError(Symbol, Symbol),
    TypeMismatch(Symbol, Symbol),
}

impl SymbolTableManager {
    pub fn new() -> Self {
        // Define Global Table
        let global_table = SymbolTable {
            parent: None,
            scopes: Vec::new(),
            symbols: HashMap::new()
        };
        // Define Manager
        Self {
            current_table_id: SymbolTableId(0),
            tables: vec![global_table]
        }
    }
    pub fn current_table_id(&mut self) -> usize {
        self.current_table_id.0
    }

    pub fn current_table_mut(&mut self) -> &mut SymbolTable {
        self.tables.get_mut(self.current_table_id.0).unwrap()
    }

    pub fn descend_scope(&mut self) {
        // Create New Scope
        let new_scope_id = SymbolTableId(self.tables.len());
        let new_scope = SymbolTable {
            parent: Some(self.current_table_id),
            scopes: Vec::new(),
            symbols: HashMap::new()
        };
        self.tables.push(new_scope);
        // Update Old Scope
        self.current_table_mut().scopes.push(new_scope_id);
        // Update Current Scope
        self.current_table_id = new_scope_id;
    }

    pub fn ascend_scope(&mut self) {
        let SymbolTableId(current_table_id) = self.current_table_id;
        let current_table = self.tables.get(current_table_id).unwrap();
        
        if let Some(parent_id) = current_table.parent {
            self.current_table_id = parent_id;
        }
    }

    pub fn lookup_mut(&mut self, name: &String) -> Option<&mut Symbol> {
        let mut current_table_id = self.current_table_id.0;
        let mut current_table = self.tables.get(current_table_id).unwrap();
        loop {
            if current_table.symbols.contains_key(name) {
                return self.tables.get_mut(current_table_id).unwrap().symbols.get_mut(name);
            } else if let Some(parent_id) = current_table.parent {
                current_table_id = parent_id.0;
                current_table = self.tables.get_mut(current_table_id).unwrap();
            } else {
                return None;
            }
        }
    }

    pub fn add_new_symbol(&mut self, symbol: Symbol) -> Result<(), self::Error> {
        // Check if name already exists
        if let Some(already_defined_symbol) = self.lookup_mut(&symbol.lexeme) {
            return Err(self::Error::RedeclareError(already_defined_symbol.clone(), symbol));
        }
        // Insert Symbol
        self.current_table_mut().symbols.insert(symbol.lexeme.clone(), symbol);
        // Return Ok
        Ok(())
    }
}
// impl SymbolTable {
    // pub fn new(parent: Option<Rc<RefCell<Box<SymbolTable>>>>) -> Self {
    //     Self {
    //         parent: parent,
    //         symbols: HashMap::new(),
    //         scopes: Vec::new(),
    //     }
    // }

    // pub fn get_parent(&self) -> Option<Rc<RefCell<Box<SymbolTable>>>> {
    //     if let Some(parent) = &self.parent {
    //         Some(parent.clone())
    //     } else {
    //         None
    //     }
    // }
    // pub fn get_scopes(&self) -> &Vec<Rc<RefCell<SymbolTable>>> {
    //     self.scopes.as_ref()
    // }
    // pub fn add_scope(&mut self, table: SymbolTable) -> Rc<RefCell<SymbolTable>> {
    //     self.scopes.push(Rc::new(RefCell::new(table)));
    //     self.scopes.last().unwrap().clone()
    // }
    // pub fn add_symbol(&mut self, name: String, symbol: Symbol) {
    //     self.symbols.insert(name, symbol);
    // }
    // pub fn lookup(&mut self, name: &String) -> Option<&mut Symbol> {
    //     if let Some(symbol) = self.symbols.get_mut(name) {
    //         return Some(symbol);
    //     } else if let Some(parent) = &self.parent {
    //         let val = parent.as_ref().borrow_mut().lookup(name);
    //         // match val {
    //         //     Some(symbol) => todo!(),
    //         //     None => todo!(),
    //         // }
    //         val
    //         // return self.parent.as_ref().and_then(|p| p.lookup(name))
    //     } else {
    //         None
    //     }
    // }
// }
