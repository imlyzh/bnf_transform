mod ast;
mod parser;
mod optimize;
mod gen;

use std::fs::File;
use std::io::prelude::*;
use std::env;

use crate::parser::*;
// use crate::ast::*;
use crate::gen::*;



fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len()-1, 2);
	let mut r = File::open(args.get(1).unwrap()).expect("input file open invalid");
	let mut buf = String::new();
	r.read_to_string(&mut buf).expect("input file open valid");
	let res = parse(&buf).expect("is not valid bnf format");
    let mut w = File::create(args.get(2).unwrap()).expect("output file open invalid");
	write!(w, "{}", res.gen()).expect("write error");
}


// #[test]
fn _test1() {
	let mut r = File::open("./test/1.bnf").unwrap();
	let mut buf = String::new();
	r.read_to_string(&mut buf).unwrap();
	let _res = parse(&buf);
	todo!("assert")
}