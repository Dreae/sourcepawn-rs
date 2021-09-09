// Copyright (C) 2021 Dreae
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// 
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Symbol(usize);

lazy_static! {
    static ref SYMBOLS: Mutex<SymbolTable> = Mutex::new(SymbolTable::new());
}

impl Symbol {
    pub fn new(name: &str) -> Symbol {
        SYMBOLS.lock().unwrap().intern(name)
    }
}

pub struct SymbolTable {
    names: HashMap<String, Symbol>,
    strings: Vec<String>
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            names: HashMap::default(),
            strings: Vec::default()
        }
    }

    pub fn intern(&mut self, string: &str) -> Symbol {
        if let Some(&symbol) = self.names.get(string) {
            return symbol;
        }

        let symbol = Symbol(self.strings.len());

        self.strings.push(string.to_owned());
        self.names.insert(string.to_owned(), symbol);
        symbol
    }
}
