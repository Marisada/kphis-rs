pub mod drg_grouper;
pub mod i10_claml;
pub mod i10_index;
pub mod i9_cm;

mod highlight;

use std::{fs::File, io::Write};
use syntect::dumps;

pub fn dump_all() {
    // Syntect Syntax Set
    println!("Create Highlight Syntax Set dump file..");
    let syntax_set = highlight::create_syntax_set();
    let syntax_set_path = "../../crates/kphis-ui-core/src/highlight/syntax_set.dump";
    dumps::dump_to_file(&syntax_set, syntax_set_path).unwrap();

    // Syntect Theme
    println!("Create Highlight Theme dump file..");
    let theme = highlight::create_theme();
    let theme_path = "../../crates/kphis-ui-core/src/highlight/theme.dump";
    dumps::dump_to_file(&theme, theme_path).unwrap();

    // DRG Grouper
    println!("Parse DRG Book2..");
    drg_grouper::book2_parser();

    println!("Parse DRG DLC Books..");
    drg_grouper::dcl_from_raw();

    println!("Create DRG grouper dump file..");
    let grouper = drg_grouper::new_grouper();
    let grouper_bytes = bitcode::encode(&grouper);
    let grouper_path = "../../crates/kphis-drg-worker/dump/grouper.dump";
    write_to(&grouper_bytes, grouper_path);

    // ICD10 WHO BOOK 1
    println!("Create WHO ICD10 2016 dump file..");
    let i10_asset = i10_claml::new_i10_claml();
    let i10_bytes = bitcode::encode(&i10_asset);
    let i10_path = "../../crates/kphis-drg-worker/dump/i10-claml.dump";
    write_to(&i10_bytes, i10_path);

    // ICD10 WHO BOOK 3, Alphabet index
    let i10_index = i10_index::parse_i10_index();
    let i10_index_bytes = bitcode::encode(&i10_index);
    let i10_index_path = "../../crates/kphis-drg-worker/dump/i10-index.dump";
    write_to(&i10_index_bytes, i10_index_path);

    println!("Done.")
}

fn write_to(bytes: &[u8], path: &str) {
    let mut file = File::create(path).unwrap_or_else(|e| {
        panic!("Error creating '{}': {}", path, e);
    });
    if let Err(e) = file.write_all(&bytes) {
        panic!("Error writing to '{}': {}", path, e);
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;

    #[test]
    fn test_dump_all() {
        dump_all();
    }
}
