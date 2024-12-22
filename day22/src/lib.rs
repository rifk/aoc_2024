use std::{collections::HashMap, iter};

use anyhow::Result;

fn next_secret(mut secret: i64) -> i64 {
    secret ^= secret * 64;
    secret %= 16777216;
    secret ^= secret / 32;
    secret %= 16777216;
    secret ^= secret * 2048;
    secret %= 16777216;
    secret
}

pub fn solve_one(input: &str) -> Result<String> {
    let monkeys = input
        .lines()
        .map(|v| Ok(v.parse::<i64>()?))
        .collect::<Result<Vec<_>>>()?;

    Ok(monkeys
        .into_iter()
        .map(|s| {
            iter::successors(Some(s), |s| Some(next_secret(*s)))
                .nth(2000)
                .unwrap()
        })
        .sum::<i64>()
        .to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    let monkeys = input
        .lines()
        .map(|v| Ok(v.parse::<i64>()?))
        .collect::<Result<Vec<_>>>()?;

    Ok(monkeys
        .clone()
        .into_iter()
        .flat_map(|s| {
            iter::successors(Some(s), |s| Some(next_secret(*s)))
                .take(2000)
                .collect::<Vec<_>>()
                .windows(2)
                .map(|w| (w[1] % 10 - w[0] % 10, w[1] % 10))
                .collect::<Vec<_>>()
                .windows(4)
                .map(|w| ((w[0].0, w[1].0, w[2].0, w[3].0), w[3].1))
                .rev()
                .collect::<HashMap<_, _>>()
        })
        .fold(HashMap::new(), |mut acc, (k, v)| {
            acc.entry(k).and_modify(|s| *s += v).or_insert(v);
            acc
        })
        .values()
        .max()
        .unwrap()
        .to_string())
}
