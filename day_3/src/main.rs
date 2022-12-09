use std::collections::hash_set::Union;
use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Debug, Clone, PartialEq)]
struct Backpack {
    idx: usize,
    items: Vec<u32>,
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
            items: items.clone(),
            left: left.to_vec(),
            right: right.to_vec(),
        };
        backpacks.push(bp);
    }

    let mut sum = 0;
    let groups = backpacks.chunks(3);
    for grp in groups {
        let mut group_set: HashSet<u32> = HashSet::new();
        let set_1: HashSet<&u32> = HashSet::from_iter(&grp[0].items);
        let set_2: HashSet<&u32> = HashSet::from_iter(&grp[1].items);
        let set_3: HashSet<&u32> = HashSet::from_iter(&grp[2].items);
        let intersect = set_1.intersection(&set_2);
        let result: Vec<u32> = intersect
            .filter(move |i| set_3.contains(*i))
            .map(|i| **i)
            .collect::<Vec<u32>>();
        println!("{:?}", result);
        sum += result[0];
    }
    println!("{:?}", sum)
}
