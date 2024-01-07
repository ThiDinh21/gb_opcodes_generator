use scraper::{ElementRef, Html, Selector};
use std::collections::HashMap;
use std::{fs::File, io::Read};

const HTML_FILE_PATH: &str = "gbops.html";

pub fn scrap_html() -> (HashMap<String, String>, HashMap<String, String>) {
    // open html file in read-only mode
    let mut file =
        File::open(HTML_FILE_PATH).expect(&format!("Failed to open file \"{}\"", HTML_FILE_PATH));

    // store file content in a string
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read file");

    let document = Html::parse_document(&content);
    let table_selector = Selector::parse("tbody").unwrap();
    let mut tables_iter = document.select(&table_selector);

    let non_prefixed_table = tables_iter.next().unwrap();
    let cb_prefixed_table = tables_iter.next().unwrap();

    (
        extract_table(non_prefixed_table),
        extract_table(cb_prefixed_table),
    )
}

fn extract_table(table_body: ElementRef) -> HashMap<String, String> {
    let mut opcode_data_map: HashMap<String, String> = HashMap::new();

    let tr_selector = Selector::parse("tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();
    let div_selector = Selector::parse("div").unwrap();

    // the index (in hex) of the row = most significant byte of the opcode
    for (msb, row) in table_body.select(&tr_selector).enumerate() {
        let row_cells = row.select(&td_selector);

        // the index (in hex) of the row = least significant byte of the opcode
        for (lsb, cell) in row_cells.enumerate() {
            // grey-outed cells / unused opcodes
            if !cell.has_children() {
                continue;
            }

            // get the hex of the opcode
            // e.g. 2E
            let mut opcode = format!("{:x}{:x}", msb, lsb);

            // get 8 or 16 bit or other
            let cell_attributes = cell.value().attr("class").unwrap();
            if cell_attributes.contains("8") {
                opcode = format!("{opcode}-8");
            } else if cell_attributes.contains("16") {
                opcode = format!("{opcode}-16");
            } else {
                opcode = format!("{opcode}-0");
            }

            // 2 layers of div tags
            let cell = cell.select(&div_selector).next().unwrap();
            let cell = cell.select(&div_selector).next().unwrap();

            let cell_data = cell.inner_html().replace("<br>", "\n");

            opcode_data_map.insert(opcode, cell_data);
        }
    }

    opcode_data_map
}
