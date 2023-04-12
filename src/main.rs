// What to scrap:
//
// 1. opcode: One or two bytes number which is an identifier of an instruction.
// 2. size: The size of instruction, including the size of immediate values.
// 3. operator: The type of operation to perform such as add and sub.
// 4. operands: The resources to be read/written by the instruction.
//              Typically, they are registers or immediate values
//              (one or two bytes number which comes after the opcode).
// 5. operand width: The information about whether this instruction handles 8-bit data or 16-bit data.
// 6. flags: The information about how the 4 flags in the flag register are updated after the instruction execution.
// 7. cycles: The number of cycles required to execute the instruction.
//            This is important when we emulate I/O devices together with CPU.
// 8. mnemonic: The assembly mnemonic of the opcode
// 9. bits: whether the opcode affect 8 or 16 (or 0) bits of memory

#![allow(unused_imports)]
extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod html_scraper;
pub mod pest_parser;

use crate::html_scraper::scrap_html;
use crate::pest_parser::parse_str;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let (non_cb, cb) = scrap_html();

    log_scrap_result(&non_cb, "scrap_non_cb.txt");
    log_scrap_result(&cb, "scrap_cb.txt");

    let mut opcode_json = HashMap::<String, HashMap<&str, String>>::new();

    for (opcode, data) in non_cb {
        // println!("{data:#?}");

        let opcode_and_bits = opcode.split("-").collect::<Vec<&str>>();
        let opcode = opcode_and_bits[0].to_uppercase().to_string();
        let bits = if opcode_and_bits.len() > 1 {
            opcode_and_bits[1].to_string()
        } else {
            "0".to_string()
        };

        let instruction = data.replace("​", " ");
        let mut parse_result = parse_str(instruction);
        parse_result.insert("bits", bits);

        opcode_json.insert(opcode, parse_result);
    }

    // Save the JSON structure into the other file.
    std::fs::write(
        "opcodes.json",
        serde_json::to_string_pretty(&opcode_json).unwrap(),
    )
    .unwrap();
}

fn log_scrap_result(non_cb: &HashMap<String, String>, filename: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
        .unwrap();

    for (opcode, data) in non_cb.iter() {
        file.write_all(opcode.to_uppercase().as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
        // replace \u200b characters with white spaces
        file.write_all(data.replace("​", " ").as_bytes()).unwrap();
        file.write_all(b"\n\n").unwrap();
    }
}
