use std::{
    cmp::{max, min},
    error::Error,
};

#[allow(dead_code)]
pub fn part1(input: &String) -> Result<(), Box<dyn Error>> {
    let elves = input
        .split("\n")
        .map(|line| {
            let [p1, p2] = line.split(",").collect::<Vec<&str>>()[..] else {
                unreachable!("There should be exactly 2 ranges")
            };

            let [start1, end1] = p1.split("-").collect::<Vec<&str>>()[..] else {
                unreachable!("There should be exactly 2 numbers (start, end)");
            };
            let [start2, end2] = p2.split("-").collect::<Vec<&str>>()[..] else {
                unreachable!("There should be exactly 2 numbers (start, end)");
            };

            let (start1, end1, start2, end2) = (
                start1.parse::<i32>().unwrap(),
                end1.parse::<i32>().unwrap(),
                start2.parse::<i32>().unwrap(),
                end2.parse::<i32>().unwrap(),
            );

            if (start1 <= start2 && end2 <= end1) || (start2 <= start1 && end1 <= end2) {
                1
            } else {
                0
            }
        })
        .sum::<i32>();

    println!("{}", elves);

    Ok(())
}

#[allow(dead_code)]
pub fn part2(input: &String) -> Result<(), Box<dyn Error>> {
    let elves = input
        .split("\n")
        .map(|line| {
            let [p1, p2] = line.split(",").collect::<Vec<&str>>()[..] else {
                unreachable!("There should be exactly 2 ranges")
            };

            let [start1, end1] = p1.split("-").collect::<Vec<&str>>()[..] else {
                unreachable!("There should be exactly 2 numbers (start, end)");
            };
            let [start2, end2] = p2.split("-").collect::<Vec<&str>>()[..] else {
                unreachable!("There should be exactly 2 numbers (start, end)");
            };

            let (start1, end1, start2, end2) = (
                start1.parse::<i32>().unwrap(),
                end1.parse::<i32>().unwrap(),
                start2.parse::<i32>().unwrap(),
                end2.parse::<i32>().unwrap(),
            );

            if max(start1, start2) <= min(end1, end2) {
                1
            } else {
                0
            }
        })
        .sum::<i32>();

    println!("{}", elves);

    Ok(())
}
