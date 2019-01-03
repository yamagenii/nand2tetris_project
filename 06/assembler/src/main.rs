#[macro_use]
extern crate lazy_static;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use crate::code::dest;
use crate::code::comp;
use crate::code::jump;


mod parser;
mod code;
mod symbol_table;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut f = File::open(filename.clone()).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    let d : Vec<&str> = contents.split('\n').collect();

    let mut symbolTable = symbol_table::SymbolTable::new();

    let mut symbold = d.clone();
    let mut symbolParser = parser::Parser::new(symbold);
    // シンボルを操作
    let mut address = 0;
    let mut asymbol = 16;
    while symbolParser.hasMoreCommands() {
        symbolParser.advance();
        match symbolParser.commandType() {
            parser::CommandType::A_COMMAND => {
                let symbol = symbolParser.symbol();
                let stri : i32 = symbol.parse().unwrap_or(-1);
                if stri == -1 {
                    if !symbolTable.contains(symbol.clone()) {
                        symbolTable.addEntry(symbol, asymbol.to_string());
                        asymbol = asymbol + 1;
                    }
                }
            },
            parser::CommandType::L_COMMAND => {
                let symbol = symbolParser.symbol();
                let stri : i32 = symbol.parse().unwrap_or(-1);
                if stri == -1 {
                    let laddress = address + 1;
                    symbolTable.addEntry(symbol, laddress.to_string());
                }
            },
            _ => (),
        }
        address = address + 1;
    }

    //symbolTable.print();
    let mut binaries: Vec<String> = Vec::new();

    let mut parser = parser::Parser::new(d);
    while parser.hasMoreCommands() {
        parser.advance();
        match parser.commandType() {
            parser::CommandType::A_COMMAND => {
                let mut symbol = parser.symbol();
                if symbolTable.contains(symbol.clone()) {
                    symbol = symbolTable.getAddress(symbol.clone());
                }
                let num: usize = symbol.parse().unwrap();
                let bsymbol = format!("{:0>15b}", num);
                let binary = format!("{}{}", "0", bsymbol);
                binaries.push(binary);

            },
            parser::CommandType::L_COMMAND => {
                let mut symbol = parser.symbol();
                if symbolTable.contains(symbol.clone()) {
                    symbol = symbolTable.getAddress(symbol.clone());
                }
                let num: usize = symbol.parse().unwrap();
                let num: usize = symbol.parse().unwrap();
                let bsymbol = format!("{:0>15b}", num);
                let binary = format!("{}{}", "0", bsymbol);
                binaries.push(binary);
            },
            parser::CommandType::C_COMMAND => {
                println!("C_Type");
                println!("comp={} dest={} jump={}", parser.comp(), parser.dest(), parser.jump());
                println!("comp={} dest={} jump={}", parser.comp(), parser.dest(), parser.jump());
                let bcomp = code::comp(parser.comp());
                let bdest = code::dest(parser.dest());
                let bjump = code::jump(parser.jump());
                let binary = format!("{}{}{}{}", "111".to_string(), bcomp, bdest, bjump);
                binaries.push(binary);
            }
            _ => (),
        }
    }

    let mut w = BufWriter::new(File::create("bin/".to_string() + filename).unwrap());

    for b in binaries {
        let s = b + "\n";
        w.write(s.as_bytes()).unwrap();
    }
}


