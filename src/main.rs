use std::fs;

fn main() {
    let url = "https://rust-lang.org/";
    let output = "rust.md";

    println!("Get url: {url}");
    let result = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&result);

    fs::write(output, md).unwrap();
    println!("Converted to file: {output}");
}