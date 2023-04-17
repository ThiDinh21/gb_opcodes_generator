use lazy_static::lazy_static;
use serde_json;
use std::fs::File;
use std::path::Path;
use std::{collections::HashMap, io::Read};
use tera::{to_value, Context, Filter, Tera, Value};

use crate::opcode_data::Opcode;

type NestedMap = HashMap<String, HashMap<String, String>>;

pub fn generate_opcodes() -> Result<(), tera::Error> {
    // set up Tera instance
    let mut tera = Tera::default();
    let mut context = Context::new();

    tera.add_template_file("templates/cpu.rs", Some("cpu.rs"))?;

    tera.register_filter("set_flag", set_flag);
    // tera.register_filter("getter", getter);
    // tera.register_filter("setter", setter);

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

    // render the template
    context.insert("opcodes", &merged_contents);
    let rendered = tera.render("cpu.rs", &context)?;

    println!("{rendered}");

    Ok(())
}

fn is_numeric(s: &str) -> bool {
    match s.trim().parse::<usize>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn set_flag(value: &Value, map: &HashMap<String, Value>) -> tera::Result<Value> {
    let v = try_get_value!("setflag", "value", String, value);
    // should be Z, N, H or C
    let flag_char = try_get_value!("setflag", "flg", String, map.get("flg").unwrap());

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
    todo!()
}

fn setter(value: &Value, map: &HashMap<String, Value>) -> tera::Result<Value> {
    todo!()
}
