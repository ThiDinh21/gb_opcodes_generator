use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub enum Cycles {
    One(usize),
    Two(usize, usize),
}

#[derive(Serialize, Deserialize)]
pub struct Opcode {
    pub code: String,
    pub mnemonic: String,
    pub operator: String,
    pub operands: Vec<String>,
    pub cycles: Cycles,
    pub size: usize,
    pub z: String,
    pub n: String,
    pub h: String,
    pub c: String,
    pub bits: usize,
}

impl Opcode {
    pub fn new(code: String, data: HashMap<String, String>, is_cb: bool) -> Self {
        let code = if is_cb {
            format!("CB{code}")
        } else {
            format!("00{code}")
        };

        let operands = data
            .get("operands")
            .unwrap()
            .split("|")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let cycles_list = data
            .get("cycles")
            .unwrap()
            .split("-")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let cycles = if cycles_list.len() == 1 {
            Cycles::One(cycles_list[0])
        } else {
            Cycles::Two(cycles_list[0], cycles_list[1])
        };

        // to make C flag distinguish from register C
        let c_flag = Self::fix_c_flag(&code, data.get("C").unwrap());

        Opcode {
            code,
            mnemonic: data.get("mnemonic").unwrap().to_string(),
            operator: data.get("operator").unwrap().to_string(),
            operands,
            cycles,
            size: Self::get_number(&data, "size"),
            z: data.get("Z").unwrap().to_string(),
            n: data.get("N").unwrap().to_string(),
            h: data.get("H").unwrap().to_string(),
            c: c_flag,
            bits: Self::get_number(&data, "bits"),
        }
    }

    fn get_number(data: &HashMap<String, String>, field: &str) -> usize {
        let data_as_str = data.get(field).unwrap();
        data_as_str.parse::<usize>().unwrap()
    }

    fn fix_c_flag(code: &str, current_cf: &str) -> String {
        if current_cf == "C"
            && (code == "0038" || code == "00D8" || code == "00DA" || code == "00DC")
        {
            "CF".to_string()
        } else {
            current_cf.to_string()
        }
    }
}

mod test {
    use super::Opcode;
    use std::collections::HashMap;

    #[test]
    fn test_constructor() {
        let data = HashMap::from([
            ("bits".to_string(), "8".to_string()),
            ("operands".to_string(), "E".to_string()),
            ("N".to_string(), "1".to_string()),
            ("H".to_string(), "H".to_string()),
            ("C".to_string(), "-".to_string()),
            ("size".to_string(), "1".to_string()),
            ("K".to_string(), "Z".to_string()),
            ("mnemonic".to_string(), "DEC E".to_string()),
            ("cycles".to_string(), "4".to_string()),
            ("operator".to_string(), "DEC".to_string()),
        ]);

        let code = "1D".to_string();

        let s = Opcode::new(code, data, true);

        assert_eq!("CB1D", &s.code);
    }
}
