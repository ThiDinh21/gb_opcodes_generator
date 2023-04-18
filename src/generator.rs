use lazy_static::lazy_static;
use serde_json;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::u16;
use std::{collections::HashMap, io::Read};
use tera::{to_value, Context, Filter, Tera, Value};

use crate::opcode_data::Opcode;

type NestedMap = HashMap<String, HashMap<String, String>>;

pub fn generate_opcodes() -> Result<(), tera::Error> {
    // set up Tera instance
    let mut tera = Tera::default();
    let mut context = Context::new();

    tera.add_template_files(vec![
        ("templates/opcodes.txt", Some("opcodes.txt")),
        ("templates/macros.txt", Some("macros.txt")),
    ])?;

    tera.register_filter("set_flag", set_flag);
    tera.register_filter("getter", getter);
    tera.register_filter("setter", setter);

    // open json files and convert them to HashMap<String, String>
    let mut opcode_json = File::open("opcodes_non_cb.json")?;
    let mut opcode_cb_json = File::open("opcodes_cb.json")?;
    let mut contents = String::new();
    let mut contents_cb = String::new();

    opcode_json.read_to_string(&mut contents)?;
    opcode_cb_json.read_to_string(&mut contents_cb)?;

    let contents: NestedMap = serde_json::from_str(&contents).unwrap();
    let contents_cb: NestedMap = serde_json::from_str(&contents_cb).unwrap();

    let mut merged_contents = Vec::<Opcode>::new();

    for (k, v) in contents {
        let opcode = Opcode::new(k, v, false);
        merged_contents.push(opcode);
    }

    for (k, v) in contents_cb {
        let opcode = Opcode::new(k, v, true);
        merged_contents.push(opcode);
    }

    merged_contents.sort_by(|a, b| hex_to_dec(&a.code).cmp(&hex_to_dec(&b.code)));

    // render the template
    context.insert("opcodes", &merged_contents);
    let rendered = tera.render("opcodes.txt", &context)?;

    // println!("{rendered}");

    write_to_file(&rendered);

    Ok(())
}

fn is_numeric(s: &str) -> bool {
    match s.trim().parse::<usize>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn set_flag(value: &Value, map: &HashMap<String, Value>) -> tera::Result<Value> {
    let v = try_get_value!("set_flag", "value", String, value);
    // should be Z, N, H or C
    let flag_char = try_get_value!("set_flag", "flg", String, map.get("flg").unwrap());

    let flag_name = match flag_char.as_ref() {
        "Z" => "ZERO",
        "N" => "SUBSTRACTION",
        "H" => "HALF_CARRY",
        "C" => "CARRY",
        _ => panic!("Invalid flag name"),
    };

    if v == "-" {
        Ok(to_value("").unwrap())
    } else if v == "0" {
        Ok(to_value(format!("self.insert(StatusFlags::{})", flag_name)).unwrap())
    } else if v == "1" {
        Ok(to_value(format!("self.clear(StatusFlags::{})", flag_name)).unwrap())
    } else {
        Ok(to_value(format!(
            "self.set(StatusFlags::{}, {})",
            flag_name,
            v.to_lowercase()
        ))
        .unwrap())
    }
}

fn getter(value: &Value, map: &HashMap<String, Value>) -> tera::Result<Value> {
    let operand = try_get_value!("argument", "value", String, value);
    let bits = try_get_value!("argument", "value", usize, map.get("bits").unwrap());
    Ok(to_value(generate_getter(&operand, bits)).expect("Error generating getter"))
}

fn setter(value: &Value, map: &HashMap<String, Value>) -> tera::Result<Value> {
    let operand = try_get_value!("argument", "value", String, value);
    let bits = try_get_value!("argument", "value", usize, map.get("bits").unwrap());
    Ok(to_value(generate_setter(&operand, bits)).expect("Error generating setter"))
}

fn generate_getter(operand: &str, bits: usize) -> String {
    if is_numeric(&operand) {
        operand.to_string()
    } else if operand.ends_with("h") {
        let mut chars = operand.chars();
        chars.next_back();
        chars.collect::<String>()
    } else if operand == "NZ" {
        format!("!self.status.contains(StatusFlags::ZERO)")
    } else if operand == "NC" {
        format!("!self.status.contains(StatusFlags::CARRY)")
    } else if operand == "Z" {
        format!("self.status.contains(StatusFlags::ZERO)")
    } else if operand == "C" {
        format!("self.status.contains(StatusFlags::ZERO)")
    } else if operand == "u8" || operand == "i8" || operand == "u16" {
        format!(
            "self.mem_read_{}(self.program_counter.wrapping_add(op_size))",
            operand
        )
    } else if operand.starts_with("FF") {
        let mut expr = operand.split("+");
        let offset = expr.next().expect("No offset");
        let num = expr.next().expect("No arg");
        format!("0x{} + {}", offset, generate_getter(num, bits))
    } else if operand.starts_with("(") {
        format!(
            "self.mem_read_u{}({})",
            bits,
            generate_getter(rm_first_last(operand), bits)
        )
    } else if operand == "SP+i8" {
        let mut expr = operand.split("+");
        let sp = expr.next().expect("No SP");
        let num = expr.next().expect("No i8");
        format!(
            "{} + {}",
            generate_getter(sp, bits),
            generate_getter(num, bits)
        )
    } else {
        // for registers + SP
        format!("self.get_{}()", operand.to_lowercase())
    }
}

fn generate_setter(operand: &str, bits: usize) -> String {
    if operand.starts_with("(") {
        format!(
            "self.mem_write_{}({})",
            bits,
            generate_getter(operand, bits)
        )
    } else {
        format!("self.set_{}", operand)
    }
}

fn rm_first_last(s: &str) -> &str {
    &s[1..s.len() - 1]
}

fn write_to_file(text: &str) {
    let filename = "output/opcodes.rs";
    let mut file = File::create(filename).expect("Can't create output file");
    file.write_all(text.as_bytes()).unwrap();
}

fn hex_to_dec(hex: &str) -> u16 {
    u16::from_str_radix(hex, 16).expect("Invalid hex string")
}
