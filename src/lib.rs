use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use lopdf::Document;

// pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     let mut results = Vec::new();
//     for line in contents.lines() {
//         if line.contains(query) {
//             results.push(line);
//         }
//     }
//     return results;
// }

// pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     let lower_case_query = query.to_lowercase();
//     let mut results = Vec::new();

//     for line in contents.lines() {
//         if line.to_lowercase().contains(&lower_case_query) {
//             results.push(line);
//         }
//     }

//     results
// }

/// this function searches a file's content with the query provided and returns a bool, if found then true and if not found then false.
pub fn search_file(file: &Path, query: &str) -> bool {
    match file.extension().and_then(|e| e.to_str()) {
        Some("txt") => search_txt_file(file, query),
        Some("pdf") => search_pdf_file(file, query),
        _ => false,
    }
}

pub fn search_pdf_file(file: &Path, query: &str) -> bool {
    let doc = Document::load(file).unwrap();
    let pages = doc.get_pages();
    for (&page_number, _) in &pages {
        let text = doc.extract_text(&[page_number]).unwrap();
        if text.contains(query) {
            return true;
        }
    }
    false
}

pub fn search_txt_file(file: &Path, query: &str) -> bool {
    let f = File::open(file).expect("failed to open file");
    let reader = BufReader::new(f);

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.contains(query) {
                return true;
            }
        }
    }
    false
}
