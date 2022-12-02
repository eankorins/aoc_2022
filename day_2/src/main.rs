use std::env;
use std::fs;

#[derive(Copy, Clone, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}
#[derive(Copy, Clone)]
struct Round {
    idx: usize,
    opponent: Shape,
    my: Shape,
}

fn outcome(round: Round) -> usize {
    let mut total = (round.my as usize) + 1;
    if round.opponent == round.my {
        total += 3;
    }
    match round.my {
        Shape::Rock => {
            if round.opponent == Shape::Scissors {
                total += 6
            }
        }
        Shape::Paper => {
            if round.opponent == Shape::Rock {
                total += 6
            }
        }
        Shape::Scissors => {
            if round.opponent == Shape::Paper {
                total += 6
            }
        }
        _ => total += 0,
    }
    total
}

fn main() {
    let content = fs::read_to_string("./input.txt").expect("Filed Should Be Read");
    let mut rounds: Vec<Round> = Vec::new();

    let split = content.split("\r\n");

    let mut cur_idx: usize = 0;

    for line in split {
        let shapes = line.split(" ").map(str::to_string).collect::<Vec<String>>();
        let op_shape = shapes[0].as_str();
        let me_shape = shapes[1].as_str();
        let op = match op_shape {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!("I'm PANICKING"),
        };
        let me = match me_shape {
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissors,
            _ => panic!("I'm PANICKING"),
        };

        let round = Round {
            idx: cur_idx,
            opponent: op,
            my: me,
        };

        rounds.push(round);
    }
    let total_score: usize = rounds.iter().map(|r| outcome(*r)).sum();
    print!("{total_score}")
}
