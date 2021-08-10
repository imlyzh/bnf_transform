mod ast;
mod optimize;
mod program;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

use crate::program::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len() - 1, 3);
    let mut r = File::open(args.get(2).unwrap()).expect("input file open invalid");
    let mut buf = String::new();
    r.read_to_string(&mut buf).expect("input file open valid");
    if args.get(1).unwrap() == "--golang" {
        let res = golang::parser::parse(&buf).expect("is not valid bnf format");
        let mut w = File::create(args.get(3).unwrap()).expect("output file open invalid");
        write!(w, "{}", golang::gen::Gen::gen(&res)).expect("write error");
    } else if args.get(1).unwrap() == "--llir" {
        let res = llir::parser::parse(&buf).expect("is not valid bnf format");
        let mut w = File::create(args.get(3).unwrap()).expect("output file open invalid");
        write!(w, "{}", llir::gen::Gen::gen(&res)).expect("write error");
    } else {
        println!("invalid style prarms");
        exit(-1);
    };
}

// #[test]
fn _test1() {
    let mut r = File::open("./test/1.bnf").unwrap();
    let mut buf = String::new();
    r.read_to_string(&mut buf).unwrap();
    let _res = golang::parser::parse(&buf);
    todo!("assert")
}
