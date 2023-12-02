#![allow(unused)]

use std::{ops::Add, fmt::format};
use regex::Regex;

const NUMBERS: [&str; 9] = [
    "one", 
    "two", 
    "three", 
    "four", 
    "five", 
    "six", 
    "seven", 
    "eight", 
    "nine", 
];


fn parse_number(s: &str) -> char {
    match s {
        "one" => '1', 
        "two" => '2', 
        "three" => '3', 
        "four" => '4', 
        "five" => '5', 
        "six" => '6', 
        "seven" => '7', 
        "eight" => '8', 
        "nine" => '9', 
        _ => panic!("invalid number")
    }
}
fn main() {

    let file = match std::fs::read_to_string("./src/input.txt") {
        Ok(x) => x,
        Err(e) => panic!("could not read file: Error {e}"),
    };

    part2(file);
}

fn part1(input: &str) -> u32  {
    let mut input_iter = input.chars();
    // some edgecase here
    // needed options for stringsonly containing 1 number
    let first = input_iter.find(|d| d.is_digit(10));
    let last = input_iter.rfind(|d| d.is_digit(10));

    match (first, last) {
        (None, None) => 0,
        (None, Some(c)) |  (Some(c), None) => {
            let s = c.to_string() + &c.to_string();
            s.parse().unwrap()
        },
        (Some(c1), Some(c2)) => (c1.to_string() + c2.to_string().as_str()).parse().unwrap(),
    }
}



fn part2(input: String) {

    let mut sum = 0;
    for line in input.lines() {

        sum += match (first_number(line), last_number(line)) {
            (Some(c1), Some(c2)) => {format!("{c1}{c2}").parse().unwrap()},
            (None, Some(c1)) => {c1.to_digit(10).unwrap()},
            (Some(c1), None) => {c1.to_digit(10).unwrap()},
            (None, None) => {0},
        };
    }

    println!("sum is: {sum}")
}


fn first_number(line: &str) -> Option<char>{

    let mut alph_number = String::new();
    for c in line.chars() {
        if c.is_digit(10) {
            return Some(c)
        }

        alph_number.push(c);
        if let Some(alph_num) = NUMBERS.iter().find(|&&x| alph_number.contains(x)){
            return Some(parse_number(alph_num)) 
        }
    };
    None
}
fn last_number(line: &str) -> Option<char>{

    // BUG will take the same number as first_number() since the both
    // get the same string and dont know about eachother
    let mut alph_number = String::new();
    for c in line.chars().rev(){

        if c.is_digit(10) {
            return Some(c)
        }

        alph_number = format!("{c}{alph_number}");

        if let Some(alph_num) = NUMBERS.iter().find(|&&x| alph_number.contains(x)){
            return Some(parse_number(alph_num)) 
        }
    };

    None

}
