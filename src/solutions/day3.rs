use std::{collections::HashSet, error::Error};

fn letter2score(c: char) -> i32 {
    if c.is_lowercase() {
        c as i32 - 'a' as i32 + 1
    } else if c.is_uppercase() {
        c as i32 - 'A' as i32 + 27
    } else {
        0
    }
}

#[allow(dead_code)]
pub fn part1(input: &String) -> Result<(), Box<dyn Error>> {
    let score = input
        .split("\n")
        .map(|backpack| {
            let (half1, half2) = (
                &backpack[0..backpack.len() / 2],
                &backpack[backpack.len() / 2..backpack.len()],
            );

            let set1: HashSet<char> = HashSet::from_iter(half1.chars());
            let set2: HashSet<char> = HashSet::from_iter(half2.chars());

            let Some(&duplicate_item) = set1.intersection(&set2).next() else {
                unreachable!("There should be exactly one duplicate item")
            };

            letter2score(duplicate_item)
        })
        .sum::<i32>();

    println!("{}", score);

    Ok(())
}

#[allow(dead_code)]
pub fn part2(input: &String) -> Result<(), Box<dyn Error>> {
    let lines = input.split("\n").collect::<Vec<&str>>();

    let score = lines
        .chunks(3)
        .map(|group_of_3| {
            let [l1, l2, l3] = group_of_3 else {
                unreachable!("Total number of lines should be divisible by 3")
            };

            let set1: HashSet<char> = HashSet::from_iter(l1.chars());
            let set2: HashSet<char> = HashSet::from_iter(l2.chars());
            let set3: HashSet<char> = HashSet::from_iter(l3.chars());

            let i12 = set1.intersection(&set2).cloned().collect::<HashSet<char>>();
            let Some(&badge) = i12.intersection(&set3).next() else {
                unreachable!("There should be exactly one badge")
            };

            letter2score(badge)
        })
        .sum::<i32>();

    println!("{}", score);

    Ok(())
}
