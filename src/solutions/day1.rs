use binary_heap_plus::*;
use std::cmp::max;
use std::error::Error;

#[allow(dead_code)]
pub fn part1(input: &String) -> Result<(), Box<dyn Error>> {
    let lines = input.split("\n");
    let mut max_elf = 0;
    let mut current_elf = 0;

    for line in lines {
        if line == "" {
            max_elf = max(max_elf, current_elf);
            current_elf = 0;
            continue;
        }

        let calories = line.parse::<i32>()?;
        current_elf += calories;
    }

    println!("{}", max_elf);

    Ok(())
}

#[allow(dead_code)]
pub fn part2(input: &String) -> Result<(), Box<dyn Error>> {
    let lines = input.split("\n");
    let mut max_three_elves = BinaryHeap::new_min();
    let mut current_elf = 0;

    for line in lines {
        if line == "" {
            if max_three_elves.len() < 3 {
                max_three_elves.push(current_elf);
            } else {
                let Some(&min_elem) = max_three_elves.peek()
                    else { unreachable!("The heap should have 3 elements!") };

                if min_elem < current_elf {
                    max_three_elves.pop();
                    max_three_elves.push(current_elf);
                }
            }

            current_elf = 0;
            continue;
        }

        let calories = line.parse::<i32>()?;
        current_elf += calories;
    }

    println!("{}", max_three_elves.iter().sum::<i32>());

    Ok(())
}
