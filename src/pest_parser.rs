use std::collections::HashMap;

use pest::Parser;

#[derive(Parser)]
#[grammar = "opcode.pest"]
pub struct OpcodeParser;

/// example instruction:
/// SCF
/// 1 4t
/// - 0 0 1
pub fn parse_str(instruction: String) -> HashMap<&'static str, String> {
    let mut res = HashMap::<&str, String>::new();

    let data_parsed_pairs = OpcodeParser::parse(Rule::Instruction, &instruction).unwrap();

    // println!("Original: {}", instruction);
    // println!("Parsed: \n{:#?}", data_parsed_pairs);
    // println!("---------------------");

    for pair in data_parsed_pairs {
        let rule = pair.as_rule();
        let text = pair.as_str().to_string();

        match rule {
            Rule::Mnemonic => {
                res.insert("mnemonic", text);

                let inner_pairs = pair.into_inner();
                let mut operands_list: Vec<String> = vec![];

                for inner_pair in inner_pairs {
                    let inner_rule = inner_pair.as_rule();
                    let inner_text = inner_pair.as_str().to_string();

                    match inner_rule {
                        Rule::Operator => {
                            res.insert("operator", inner_text);
                        }
                        Rule::Operand => {
                            operands_list.push(inner_text);
                        }
                        _ => (),
                    }
                }

                res.insert("operands", operands_list.join("|"));

                None
            }
            Rule::Number => res.insert("size", text),
            Rule::Cycles => res.insert("cycles", text.replace("t", "")),
            Rule::Flags => {
                let inner_pairs = pair.into_inner();

                for (i, inner_pair) in inner_pairs.enumerate() {
                    let flag_value = get_flag_value(inner_pair.as_rule());

                    match i {
                        0 => res.insert("K", flag_value),
                        1 => res.insert("N", flag_value),
                        2 => res.insert("H", flag_value),
                        3 => res.insert("C", flag_value),
                        _ => None,
                    };
                }

                None
            }
            _ => None,
        };
    }

    res
}

fn get_flag_value(rule: Rule) -> String {
    match rule {
        Rule::Z => "Z",
        Rule::N => "N",
        Rule::H => "H",
        Rule::C => "C",
        Rule::NotAffect => "-",
        Rule::Set => "1",
        Rule::Unset => "0",
        _ => panic!("Flag parsed error"),
    }
    .to_string()
}
