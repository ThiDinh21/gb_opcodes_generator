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
// 8. group: Which group the opcode belongs to

pub mod html_scraper;

use crate::html_scraper::scrap_html;

fn main() {
    let (non_cb, cb) = scrap_html();
    
}
