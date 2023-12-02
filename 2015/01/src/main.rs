#![allow(unused)]

fn main() {

    let file = match std::fs::read_to_string("./src/input.txt") {
        Ok(x) => x,
        Err(e) => panic!("could not read file: Error {e}"),
    };

    let mut santa = Santa::new();
    //part1(&mut santa, file);

    
    let enter_basement = part2(&mut santa, file);
    println!("{}", enter_basement);

}

struct Santa {
    cur_floor:i32,
}

impl Santa {
    fn new() -> Self {
        Santa { cur_floor: 0 }
    }
    
    fn walk_floor(&mut self, char: char)  {
        match char {
            '(' => self.cur_floor += 1,
            ')' => self.cur_floor -= 1, 
            _ => panic!("Invalid walk command")
        }
    }
}
fn part1(santa: &mut Santa, movement_seq: String) {

    for char in movement_seq.chars(){

        santa.walk_floor(char)
    }
}
fn part2(santa: &mut Santa, movement_seq: String) -> usize{


    for (index, char) in movement_seq.chars().enumerate(){
        
        santa.walk_floor(char);

        if santa.cur_floor == -1 {
            return index + 1
        }
    }
    0
}


