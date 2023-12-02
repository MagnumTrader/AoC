#![allow(unused)]


fn main() {

    let file = match std::fs::read_to_string("./src/input.txt") {
        Ok(x) => x,
        Err(e) => panic!("could not read file: Error {e}"),
    };



    // each pich is separated by ; each cube by ,
    // what games are possible wi&fileth 12 red cubes 13 green cubes and 14 blue
    part1(&file);
    
    //part2()

}

fn parse_games(input: &str) -> Vec<Game> {
    let mut games = vec![];
    for line in input.lines() {
        let s = line.split(": ").collect::<Vec<&str>>();

        let s = Game {
            number: s[0].split(" ").last().unwrap().parse().unwrap(),
            rounds: s[1].split("; ").map(|round| round_from_str(round)).collect(),
        };
        games.push(s);
    }
    
    games
}
fn part1(input: &str) {

    let games = parse_games(input);

    let sum: i32 = games
        .iter()
        .filter(|game| game.is_possible(Round { red: 12, blue: 14, green: 13 }))
        .map(|game| game.number)
        .sum();
    
    println!("sum is of game numbers is: {sum}");
        
}

fn part2(someinput: i32) {

}

#[derive(Debug)]
struct Game {
    number: i32,
    rounds: Vec<Round>
}
impl Game {
    fn is_possible(&self, round: Round) -> bool  {
        for r in &self.rounds {

            println!("{r:?}");
            println!("{round:?}");


            if r.red > round.red || r.green > round.green || r.blue > round.blue {
                println!("this game is not possible");
                return false
            }
        }

        true
    }
}

#[derive(Debug)]
struct Round {
    red: i32,
    blue: i32,
    green: i32,

}
impl Round {
    fn new() -> Self  {
        Self { 
            red: i32::default(), 
            blue: i32::default(), 
            green: i32::default() 
        }
    }
}
fn round_from_str(input: &str) -> Round {

    let cubes_in_round: Vec<Cube> = input.split(", ").map(|cube| Cube::from_str(cube)).collect();
    let mut round = Round::new();
    for cube in cubes_in_round {
        match cube {
            Cube::Red(x) => round.red = x,
            Cube::Green(x) => round.green = x,
            Cube::Blue(x) => round.blue = x,
        }
    }

    return round
}
#[derive(Debug)]
enum Cube {
    Red(i32),
    Green(i32),
    Blue(i32)
}
impl Cube {
    fn from_str(s: &str) -> Cube {

        let mut cube_iter = s.split(" ");
        let quantity = cube_iter.next().unwrap().parse().unwrap();

        match cube_iter.next().unwrap() {
            "blue" => Cube::Blue(quantity),
            "red" => Cube::Red(quantity),
            "green" => Cube::Green(quantity),
            _ => {panic!("could not parse cube from str")},
        }
    }
}





