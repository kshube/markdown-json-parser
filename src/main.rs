pub mod parser;
pub mod translator;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug)]
pub enum Markdown {
  Heading(Vec<String>),
  Paragraph(Vec<String>),
}

fn usage() {
  println!("Usage: {} <markdown>.md <newfile>.json\n", String::from(env!("CARGO_PKG_NAME")));
}

fn parse_markdown_file(_input_filename: &str, _output_filename: &str) {
  let input_file_path = Path::new(_input_filename);
  let output_file_path = Path::new(_output_filename);

  let mut input_file = File::open(&input_file_path)
            .expect("ERROR : Failed to open file");
  let mut input_file_content = String::new();
  input_file.read_to_string(&mut input_file_content)
            .expect("ERROR : Failed to read file");
  let out = match parser::parse_markdown(&input_file_content) {
    Ok((_, content)) => {
      // println!("{:?}", content);
      translator::translate(content)
    },
    Err(_) => String::from("ERROR : Failed to parse file"),
  };
  let mut output_file = File::create(&output_file_path)
            .expect("ERROR : Failed to create file");
  output_file.write_all(out.as_bytes())
            .expect("ERROR : Failed to write to file");
}

fn main() {
  let args: Vec<String> = std::env::args().collect();
  match args.len() {
    3 => parse_markdown_file(&args[1], &args[2]),
    _ => usage()
  }
}