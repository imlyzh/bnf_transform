mod ast;
mod parser;
mod gen;

use std::fs::File;
use std::io::prelude::*;

use crate::parser::parse;

fn main() {
	let mut r = File::open("./test/1.bnf").unwrap();
	let mut buf = String::new();
	r.read_to_string(&mut buf).unwrap();
	let res = parse(&buf);
	println!("out: {:?}", res);
}
