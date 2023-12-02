#![allow(unused)]

use std::{borrow::BorrowMut, ops::Sub};
fn main() {

    let file = match std::fs::read_to_string("./src/input.txt") {
        Ok(x) => x,
        Err(e) => panic!("could not read file: Error {e}"),
    };

    let mut streetlights = vec![vec![StreetLight(0); 1000];1000];

    for line in file.lines(){
        let (start, end, command) = parse_range_and_command(line);

        toggle_range(&mut streetlights, command, (start, end))

    }

    // forgot to fold with the iteratorjj
    let total_brightness = streetlights
        .iter()
        .fold(0, |acc, row| acc + row
            .iter()
            .map(|light| light.0)
            .sum::<u32>()
        );

    println!("{total_brightness}");

}

fn toggle_range(sl: &mut Vec<Vec<StreetLight>>, command: Command, range: ((usize, usize),(usize, usize))) {

    let start = (range.0.0, range.0.1);
    let end = (range.1.0, range.1.1);

    // Get the reference to a cell to change then do based on command
    // TODO i could specify the range and only pass that range to the function and light it all
    for i in start.0..=end.0 {
        // all the rows
        for j in start.1..=end.1{
            // all the columns
            // toggle that light
            let mut light = sl[i][j].borrow_mut();
            match command {
                Command::On => light.on(),
                Command::Off => light.off(),
                Command::Toggle => light.toggle(),
            }
        }
    }
}

fn parse_range_and_command(s: &str) -> ((usize, usize), (usize, usize), Command)  {
    // split on through
    //
    // split on space
    let s: String = s.split("through ").collect();
    let s: Vec<&str> = s.rsplitn(3, " ").collect();
    let mut startsplit = s[1].split(",");
    let start: (usize, usize) = (
        startsplit.next().unwrap().parse().unwrap(), 
        startsplit.next().unwrap().parse().unwrap()
    );

    let mut endsplit = s[0].split(",");
    let end: (usize, usize) = (
        endsplit.next().unwrap().parse().unwrap(), 
        endsplit.next().unwrap().parse().unwrap()
    );

    let command = s.last().unwrap();

    (start, end, Command::from_str(command).unwrap())
}




fn part2(someinput: i32) {

}

#[derive(Debug)]
enum Command {
    On,
    Off,
    Toggle
}

impl Command {
    fn from_str(s:&str) -> Result<Self, String> {
        match s.to_lowercase().trim(){
            "toggle" => Ok(Command::Toggle),
            "turn on" => Ok(Command::On),
            "turn off" => Ok(Command::Off),
            _ => Err("Could not parse command, unkown".to_owned())
        }
    }
}
#[derive(Debug, Clone, Copy)]
struct StreetLight(u32);
impl StreetLight {
    fn toggle(&mut self){
        self.0 += 2;
    }
    fn on (&mut self){
        self.0 += 1;
    }
    fn off (&mut self){
        if self.0 == 0 {
            return;
        }

        self.0 -= 1;

    }
}
