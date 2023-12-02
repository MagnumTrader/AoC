#![allow(unused)]

use std::{slice::Chunks, collections::{HashSet, HashMap}};



fn main() {

    let file = match std::fs::read_to_string("./src/input.txt") {
        Ok(x) => x,
        Err(e) => panic!("could not read file: Error {e}"),
    };

    let mut nice_and_naughty2 = (0, 0);

    for line in file.lines() {
        match is_nice2(line) {
            true =>{
                nice_and_naughty2.0 += 1
            } ,
            false => {
                nice_and_naughty2.1 += 1
            },
        }
    }

    println!("nice: {} naughty: {}",  nice_and_naughty2.0, nice_and_naughty2.1);
    // Iter over that counts if its a bad string, fold value into tuple

// Nice string
    //three vowels
    //at least one letter that apperas twice in a row 'aa'
    //does note contain ab, cd ,pq or xy
// else naughty sting

}


const VOWELS: &str = "aeiou";
const ILLEGAL_COMBO: [&str; 4] = ["ab", "cd", "pq", "xy"];

fn is_nice(s: &str) -> bool {

    let mut in_a_row = false;
    let mut vowels_count = 0;

    for (i, char1) in s.chars().enumerate(){

        if VOWELS.contains(char1){
            vowels_count += 1;
        }

        // check that char2 is not out of bounds
        if let Some(char2) = s.chars().nth(i + 1) {

            let charpair = format!("{}{}", char1, char2);

            // return early if illegal combo found
            if ILLEGAL_COMBO.contains(&charpair.as_str()){
                return false;
            };

            if char1 == char2 {
                in_a_row = true;
            }
        }
    }

    in_a_row && vowels_count >= 3 
}

fn is_nice2(s: &str) -> bool {

    // parir : (foundtimes, lastindex)
    let mut pairs: HashMap<String, (i32, i32)> = HashMap::new();
    let mut separate_doubles = false;

    for (i, char1) in s.chars().enumerate(){
        // check if pair in 
        if let Some(char3) =  s.chars().nth(i + 2) {

            if char1 == char3 {
                separate_doubles = true;
            } 
        }
        if let Some(char2) = s.chars().nth(i + 1) {

            let pair = format!("{}{}", char1, char2);
            let pairtup = pairs.entry(pair).or_insert((0, i as i32 + 1));

            if i as i32 != pairtup.1 {
                pairtup.0 += 1;
            }
        }
    }
    
    let enough_pairs = pairs.iter().any(|(_, x)| x.0 > 1);
    enough_pairs && separate_doubles
}



fn part2(someinput: i32) {

}

// FIRST INPUTS
// sszojmmrrkwuftyv
// isaljhemltsdzlum
// fujcyucsrxgatisb
// qiqqlmcgnhzparyg
// oijbmduquhfactbc
// jqzuvtggpdqcekgk
// ...
