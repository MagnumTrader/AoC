#![allow(unused)]
use std::collections::HashMap;
fn main() {

    let file = match std::fs::read_to_string("./src/input.txt") {
        Ok(x) => x,
        Err(e) => panic!("could not read file: Error {e}"),
    };

    
    //part1()
    println!("{:?}", part2(file))

    
    //part2()

}

// I should be able to implement adding for my tuple
//
type Coordinate = (i16, i16);

impl AddTuple for Coordinate {
    fn new(init: (i16, i16)) -> Self{
        init
    }
    fn add (&mut self, other: (i16, i16)){
        self.0 += other.0;
        self.1 += other.1;
    }
}

trait AddTuple {
    fn new(init: (i16, i16)) -> Self;
    fn add (&mut self, other: (i16, i16));
}

fn part1(data: String) -> usize {

    let mut current_coordinate = Coordinate::new((0,0));
    let mut houses_visited: HashMap<Coordinate, i16> = HashMap::new();
    houses_visited.insert(current_coordinate, 1);

    // check the direction in the each char of data
    for char in data.chars() {

        current_coordinate.add(parse_coordinate(char));
        *houses_visited.entry(current_coordinate).or_insert(0) += 1;
        // insert or add to the hashmap
    }

    houses_visited.len()
}

fn part2(data: String) -> usize {

    let mut santa_coordinates = Coordinate::new((0,0));
    let mut robot_coordinates = Coordinate::new((0,0));

    let mut houses_visited: HashMap<Coordinate, i16> = HashMap::new();
    houses_visited.insert((0,0), 1);

    let len = data.len();
    let mut data = data.chars();
    loop {
        match (data.next(), data.next()) {
            (Some(s), Some(r)) => {

                santa_coordinates.add(parse_coordinate(s));
                *houses_visited.entry(santa_coordinates).or_insert(0) += 1;
                
                robot_coordinates.add(parse_coordinate(r));
                *houses_visited.entry(robot_coordinates).or_insert(0) += 1;
            },
            (Some(s), None) => {

                santa_coordinates.add(parse_coordinate(s));
                *houses_visited.entry(santa_coordinates).or_insert(0) += 1;

            },
            (None, Some(r)) => {

                robot_coordinates.add(parse_coordinate(r));
                *houses_visited.entry(robot_coordinates).or_insert(0) += 1;

            }
            (None, None) => break,
        }
    }
    houses_visited.len()
}
fn parse_coordinate (c: char) -> (i16, i16){
    match c {
        // Match and add to that coordinate
        '^' => (0,1),
        'v' => (0,-1),
        '>' => (1,0),
        '<' => (-1,0),
        invalid => panic!("Invalid input for movement {invalid} expects [^<>v] from string")
    }
}
