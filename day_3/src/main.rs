use std::collections::hash_set::Union;
use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Debug, Clone, PartialEq)]
struct Backpack {
    idx: usize,
    left: Vec<u32>,
    right: Vec<u32>,
}

fn main() {
    let content = fs::read_to_string("./input.txt").expect("Filed Should Be Read");
    let mut backpacks: Vec<Backpack> = Vec::new();

    let split = content.split("\r\n");
    for line in split {
        let mut items = vec![];
        let chars = line.chars();
        for char in chars {
            let ascii = char as u32;
            if ascii >= 65 && ascii <= 90 {
                items.push((ascii - 64) + 26);
            } else {
                items.push(ascii - 96)
            }
        }
        let half = items.len() / 2;
        let (left, right) = items.split_at(half);

        let bp = Backpack {
            idx: 1,
            left: left.to_vec(),
            right: right.to_vec(),
        };
        backpacks.push(bp);
    }

    let mut sum = 0;
    for bp in backpacks {
        println!("{:?}", bp);
        let left_set: HashSet<u32> = HashSet::from_iter(bp.left);
        let right_set: HashSet<u32> = HashSet::from_iter(bp.right);
        let union = left_set.intersection(&right_set);
        let bp_sum: u32 = union.sum();
        sum += bp_sum;
    }
    println!("{:?}", sum)
}
