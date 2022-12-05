use regex::Regex;
use std::{collections::VecDeque, error::Error};

#[allow(dead_code)]
pub fn part1(input: &String) -> Result<(), Box<dyn Error>> {
    let [crates, actions] = input.split("\n\n").collect::<Vec<&str>>()[..] else {
        unreachable!();
    };

    let crate_no = (crates.split("\n").collect::<Vec<&str>>()[0].len() + 1) / 4;
    let mut crate_stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); crate_no];

    for line in crates.split("\n").take(crates.split("\n").count() - 1) {
        for i in 0..crate_no {
            let crate_starting_point = i * 3 + i;
            if &line[crate_starting_point..=crate_starting_point + 2] != "   " {
                crate_stacks[i].push_back(line.as_bytes()[i * 3 + i + 1] as char);
            }
        }
    }

    let re =
        Regex::new(r"move (?P<crates>[0-9]+) from (?P<from>[0-9]+) to (?P<to>[0-9]+)").unwrap();
    for line in actions.split("\n") {
        let caps = re.captures(line).unwrap();
        let crates = caps.name("crates").unwrap().as_str().parse::<usize>()?;
        let from = caps.name("from").unwrap().as_str().parse::<usize>()?;
        let to = caps.name("to").unwrap().as_str().parse::<usize>()?;

        for _ in 0..crates {
            if let Some(popped) = crate_stacks[from - 1].pop_front() {
                crate_stacks[to - 1].push_front(popped);
            } else {
                break;
            }
        }
    }

    let result = crate_stacks.iter().map(|s| s[0]).collect::<String>();
    println!("{}", result);

    Ok(())
}

#[allow(dead_code)]
pub fn part2(input: &String) -> Result<(), Box<dyn Error>> {
    let [crates, actions] = input.split("\n\n").collect::<Vec<&str>>()[..] else {
        unreachable!();
    };

    let crate_no = (crates.split("\n").collect::<Vec<&str>>()[0].len() + 1) / 4;
    let mut crate_stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); crate_no];

    for line in crates.split("\n").take(crates.split("\n").count() - 1) {
        for i in 0..crate_no {
            let crate_starting_point = i * 3 + i;
            if &line[crate_starting_point..=crate_starting_point + 2] != "   " {
                crate_stacks[i].push_back(line.as_bytes()[i * 3 + i + 1] as char);
            }
        }
    }

    let re =
        Regex::new(r"move (?P<crates>[0-9]+) from (?P<from>[0-9]+) to (?P<to>[0-9]+)").unwrap();
    for line in actions.split("\n") {
        let caps = re.captures(line).unwrap();
        let crates = caps.name("crates").unwrap().as_str().parse::<usize>()?;
        let from = caps.name("from").unwrap().as_str().parse::<usize>()?;
        let to = caps.name("to").unwrap().as_str().parse::<usize>()?;

        let mut temporary_crate_stack = VecDeque::new();
        for _ in 0..crates {
            if let Some(popped) = crate_stacks[from - 1].pop_front() {
                temporary_crate_stack.push_front(popped);
            } else {
                break;
            }
        }
        for c in temporary_crate_stack {
            crate_stacks[to - 1].push_front(c);
        }
    }

    let result = crate_stacks.iter().map(|s| s[0]).collect::<String>();
    println!("{}", result);

    Ok(())
}
