extern crate getopts;
use getopts::Options;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    
    let mut opts = Options::new();
    opts.reqopt("f", "from", "begining number", "FROM");
    opts.reqopt("t", "to", "ending number", "TO");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string())
    };

    let from = matches.opt_get_default("f", 0).unwrap();
    let to = matches.opt_get_default("to", 20).unwrap();
    fizzbuzz(from, to);
}

fn fizzbuzz(from: usize, to: usize) {
    for number in from ..= to {
        if number % 15 == 0 {
            println!("fizzbuzz ");
        } else if number % 3 == 0 {
            println!("fizz ");
        } else if number % 5 == 0 {
            println!("buzz ");
        } else {
            println!("{} ", number);
        }
    }
}