#![allow(unused)]
fn main() {

    let file = match std::fs::read_to_string("./src/input.txt") {
        Ok(x) => x,
        Err(e) => panic!("could not read file: Error {e}"),
    };

    let mut presents: Vec<Present> = file.lines().map(|present| {
        let mut dim = present.split("x");
        vec![
            dim.next().unwrap().parse().unwrap(),
            dim.next().unwrap().parse().unwrap(),
            dim.next().unwrap().parse().unwrap(),
        ]
    }).collect();

    //part1()
    
    let total_paper: i32 = presents.iter().map(|present| part1(&present)).sum();
    println!("total paper needed: {total_paper} square feet");
    //part2()

    let total_ribbon: i32 = presents.iter_mut().map(|mut present| part2(&mut present)).sum();
    println!("total ribbon needed: {total_ribbon} feet");
}

type Present = Vec<i32>;

fn part1(present: &Present) -> i32 {

    let mut unique_sides = Vec::new();
    present
        .iter()
        .enumerate()
        .for_each(|(index, m1)|{
            present[(index + 1)..]
                .iter()
                .for_each(|m2|{
                    unique_sides.push(m1 * m2);
                });
        });

    let slack = unique_sides.iter().min().unwrap();
    unique_sides.iter().map(|x| x*2 ).sum::<i32>() + slack
}

fn part2(present: &mut Present) -> i32{

    present.sort_unstable();

    let bow: i32 = present.iter().product();

            //let sum = a.iter().fold(0, |acc, x| acc + x);

    let length: i32 = present.iter().take(2).map(|m| m * 2).sum();
    length + bow
}

