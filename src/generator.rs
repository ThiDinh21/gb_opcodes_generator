fn is_numeric(s: &str) -> bool {
    match s.trim().parse::<usize>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn generate_opcodes() {}
