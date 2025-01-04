use anyhow::{bail, Result};

fn parse_input(input: &str) -> Result<(Vec<[usize; 5]>, Vec<[usize; 5]>)> {
    let (locks, keys): (Vec<_>, Vec<_>) = input
        .split("\n\n")
        .partition(|block| block.starts_with("#####\n"));

    Ok((
        locks
            .into_iter()
            .map(|block| {
                let mut lock = [0; 5];
                for line in block.lines().skip(1) {
                    for i in 0..5 {
                        if line[i..=i] == *"#" {
                            lock[i] += 1;
                        }
                    }
                }
                lock
            })
            .collect(),
        keys.into_iter()
            .map(|block| {
                let mut key = [0; 5];
                for line in block.lines().rev().skip(1) {
                    for i in 0..5 {
                        if line[i..=i] == *"#" {
                            key[i] += 1;
                        }
                    }
                }
                key
            })
            .collect(),
    ))
}

pub fn solve_one(input: &str) -> Result<String> {
    let (locks, keys) = dbg!(parse_input(input)?);

    Ok(locks
        .into_iter()
        .map(|lock| {
            keys.iter()
                .filter(|key| {
                    key[0] + lock[0] <= 5
                        && key[1] + lock[1] <= 5
                        && key[2] + lock[2] <= 5
                        && key[3] + lock[3] <= 5
                        && key[4] + lock[4] <= 5
                })
                .count()
        })
        .sum::<usize>()
        .to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    bail!("no day25 part2")
}
