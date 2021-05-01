mod ast;
mod parser;
mod optimize;
mod gen;

use std::fs::File;
use std::io::prelude::*;

use crate::parser::*;
// use crate::ast::*;
use crate::gen::*;

fn main() {
	let mut r = File::open("./doc/bnf.bnf").unwrap();
	let mut buf = String::new();
	r.read_to_string(&mut buf).unwrap();
	let res = parse(&buf);
	println!("out: {:?}", res);
	println!("---------------------------------------------");
	println!("gen: {}", res.unwrap().gen());
}


// #[test]
fn test1() {
	let mut r = File::open("./test/1.bnf").unwrap();
	let mut buf = String::new();
	r.read_to_string(&mut buf).unwrap();
	let _res = parse(&buf);
	todo!("assert")
}