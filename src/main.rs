pub mod rn;
pub mod util;

use core::panic;
use std::convert::From;
use rn::RomanNumber;
use util::separator;

fn main() {
    loop {
        println!("Ctrl + C to exit.");
        println!();
        examples();
        description();
        you_input();
        separator('_', 100);
    }
}

fn examples() {
    println!("Examples");
    println!("III (3): {}", text_roman_to_int(String::from("III")));
    println!("LVIII (58): {}", text_roman_to_int(String::from("LVIII")));
    println!("MCMXCIV (1994): {}", text_roman_to_int(String::from("MCMXCIV")));
}

fn description() {
    let buf = vec![b'0', b'0', b'0'];
    let s = match String::from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    println!("Roman digits: I = 1, V = 5, X = 10, L = 50, C = 100, D = 500, M = 1{}", s);
}

fn you_input() {
    println!("Input you:");
    let input = readln!();
    separator('_', 100);
    println!("{}: {}", input.to_ascii_uppercase(), text_roman_to_int(String::from(input)));
}

fn text_roman_to_int(s: String) -> i32 {
    let num = RomanNumber::from(s);
    num.to_i32()
}
