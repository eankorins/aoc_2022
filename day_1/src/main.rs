use std::env;
use std::fs;

#[derive(Copy, Clone)]
struct Elf {
    idx: usize,
    calories: usize,
}

fn main() {
    let content = fs::read_to_string("./input.txt").expect("Filed Should Be Read");
    let mut elves: Vec<Elf> = Vec::new();

    let split = content.split("\r\n");

    let mut cur_idx: usize = 0;
    let mut elf: Elf = Elf {
        idx: cur_idx,
        calories: 0,
    };

    for line in split {
        if line.is_empty() {
            elves.push(elf);
            let msg = format!("{} {}", elves[cur_idx].idx, elves[cur_idx].calories);
            println!("{msg}");
            cur_idx = cur_idx + 1;
            elf = Elf {
                idx: cur_idx,
                calories: 0,
            };
        } else {
            elf.calories = elf.calories + line.parse::<usize>().unwrap();
        }
    }
    elves.push(elf);
    elves.sort_by(|a, b| b.calories.cmp(&a.calories));
    let best_elf = elves[0];

    let msg = format!("{} {}", best_elf.idx, best_elf.calories);
    println!("{msg}");
}
