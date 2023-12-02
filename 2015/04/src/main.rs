#![allow(unused)]
use md5;
use std::fmt::LowerHex;
fn main() {

    let file = match std::fs::read_to_string("./src/input.txt") {
        Ok(x) => x,
        Err(e) => panic!("could not read file: Error {e}"),
    };

    //part1()
    
    // trim because \n
    part2(file.trim().to_string(), 6);
    //part2()

}

fn part1(key: String) {

    let mut try_nr = 0;
    let mut first5 = " ".to_string();

    loop{

        let crypt = md5::compute(format!("{key}{try_nr}"));
        first5 = format!("{:?}", crypt).chars().take(5).collect();

        if first5 == "00000".to_string(){
            break;
        }
        try_nr += 1;
    }
    // TODO mitt problem just nu är att jag inte vet hur jag går ut strängen från Digest

    println!("input is now: {try_nr}")

}

fn part2(key: String, leading_0s: usize) {

    //repeat once instead of every lap since this doesnt change
    let zeros = "0".repeat(leading_0s);

    let mut try_nr = 0;
    let mut first5 = String::new();

    loop{

        let crypt = md5::compute(format!("{key}{try_nr}"));
        first5 = format!("{:?}", crypt);

        if &first5[..leading_0s] == zeros{
            break;
        }
        try_nr += 1;
    }

    println!("input is now: {try_nr}")
}

