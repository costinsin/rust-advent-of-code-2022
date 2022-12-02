use std::error::Error;

#[allow(dead_code)]
pub fn part1(input: &String) -> Result<(), Box<dyn Error>> {
    let lines = input.split("\n");
    let mut points = 0;

    for line in lines {
        let moves = line.split(" ").collect::<Vec<&str>>();
        let (oponent, me) = (
            moves[0].chars().nth(0).unwrap() as i32 - 'A' as i32,
            moves[1].chars().nth(0).unwrap() as i32 - 'X' as i32,
        );

        if oponent == me {
            points += 3;
        } else if (me == 0 && oponent == 2)
            || (me == 1 && oponent == 0)
            || (me == 2 && oponent == 1)
        {
            points += 6;
        }

        points += match me {
            0 => 1,
            1 => 2,
            2 => 3,
            _ => 0,
        };
    }

    println!("{}", points);

    Ok(())
}

#[allow(dead_code)]
pub fn part2(input: &String) -> Result<(), Box<dyn Error>> {
    let lines = input.split("\n");
    let mut points = 0;

    for line in lines {
        let moves = line.split(" ").collect::<Vec<&str>>();
        let (oponent, action) = (
            moves[0].chars().nth(0).unwrap() as i32 - 'A' as i32,
            moves[1].chars().nth(0).unwrap() as i32 - 'X' as i32,
        );

        points += match action {
            0 => 0,
            1 => 3,
            2 => 6,
            _ => 0,
        };

        points += match action {
            0 => match oponent {
                0 => 3,
                1 => 1,
                2 => 2,
                _ => 0,
            },
            1 => match oponent {
                0 => 1,
                1 => 2,
                2 => 3,
                _ => 0,
            },
            2 => match oponent {
                0 => 2,
                1 => 3,
                2 => 1,
                _ => 0,
            },
            _ => 0,
        };
    }

    println!("{}", points);

    Ok(())
}
