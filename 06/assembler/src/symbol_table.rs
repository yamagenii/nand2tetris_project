use std::collections::HashMap;

pub struct SymbolTable {
    symbolTable : HashMap<String, String>,

}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        let mut table = SymbolTable {
            symbolTable : HashMap::new(),
        };
        table.symbolTable.insert("SP".to_string(), "0".to_string());
        table.symbolTable.insert("LCL".to_string(), "1".to_string());
        table.symbolTable.insert("ARG".to_string(), "2".to_string());
        table.symbolTable.insert("THIS".to_string(), "3".to_string());
        table.symbolTable.insert("R0".to_string(), "0".to_string());
        table.symbolTable.insert("R1".to_string(), "1".to_string());
        table.symbolTable.insert("R2".to_string(), "2".to_string());
        table.symbolTable.insert("R3".to_string(), "3".to_string());
        table.symbolTable.insert("R4".to_string(), "4".to_string());
        table.symbolTable.insert("R5".to_string(), "5".to_string());
        table.symbolTable.insert("R6".to_string(), "6".to_string());
        table.symbolTable.insert("R7".to_string(), "7".to_string());
        table.symbolTable.insert("R8".to_string(), "8".to_string());
        table.symbolTable.insert("R9".to_string(), "9".to_string());
        table.symbolTable.insert("R10".to_string(), "10".to_string());
        table.symbolTable.insert("R11".to_string(), "11".to_string());
        table.symbolTable.insert("R12".to_string(), "12".to_string());
        table.symbolTable.insert("R13".to_string(), "13".to_string());
        table.symbolTable.insert("R14".to_string(), "14".to_string());
        table.symbolTable.insert("R15".to_string(), "15".to_string());
        table.symbolTable.insert("SCREEN".to_string(), "16384".to_string());
        table.symbolTable.insert("KBD".to_string(), "24576".to_string());
        return table;
    }

    pub fn addEntry(&mut self, symbol : String, address : String) {
        self.symbolTable.insert(symbol, address);
    }

    pub fn contains(&mut self, symbol : String) -> bool {
        return self.symbolTable.contains_key(&symbol);
    }

    pub fn getAddress(&mut self, symbol : String) -> String {
        let d = &"".to_string();
        return self.symbolTable.get(&symbol).unwrap_or(d).to_string();
    }
    pub fn print(&mut self) {
        for (k, v) in self.symbolTable.iter() {
            println!("{} : {}", k, v);
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn B01() {
        let mut table = SymbolTable::new();
        table.addEntry("aaa".to_string().to_string(), "0111".to_string());
        assert_eq!(table.contains("aaa".to_string()), true);
        assert_eq!(table.contains("aba".to_string()), false);
        assert_eq!(table.contains("R0".to_string()), true);
        assert_eq!(table.getAddress("aaa".to_string()), "0111".to_string());
        assert_eq!(table.getAddress("R0".to_string()), "0".to_string());
    }
}
