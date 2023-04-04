use scraper::Html;
use std::{fs::File, io::Read};

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

fn main() {
    const HTML_FILE_PATH: &str = "gbops.html";

    // open html file in read-only mode
    let mut file =
        File::open(HTML_FILE_PATH).expect(&format!("Failed to open file \"{}\"", HTML_FILE_PATH));

    // store file content in a string
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read file");

    let document = Html::parse_document(&content);
}
