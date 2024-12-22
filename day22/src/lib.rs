use std::{collections::HashMap, iter};

use anyhow::Result;

const PRUNE_MASK: i32 = 2i32.pow(24) - 1;
fn next_secret(mut secret: i32) -> i32 {
    secret ^= secret << 6;
    secret &= PRUNE_MASK;
    secret ^= secret >> 5;
    secret &= PRUNE_MASK;
    secret ^= secret << 11;
    secret &= PRUNE_MASK;
    secret
}

pub fn solve_one(input: &str) -> Result<String> {
    let monkeys = input
        .lines()
        .map(|v| Ok(v.parse::<i32>()?))
        .collect::<Result<Vec<_>>>()?;

    Ok(monkeys
        .into_iter()
        .map(|s| {
            iter::successors(Some(s), |s| Some(next_secret(*s)))
                .nth(2000)
                .unwrap() as u64
        })
        .sum::<u64>()
        .to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    let monkeys = input
        .lines()
        .map(|v| Ok(v.parse::<i32>()?))
        .collect::<Result<Vec<_>>>()?;

    Ok(monkeys
        .clone()
        .into_iter()
        .flat_map(|s| {
            iter::successors(Some(s), |s| Some(next_secret(*s)))
                .map(|s| s % 10)
                .take(2000)
                .collect::<Vec<_>>()
                .windows(2)
                .map(|w| (w[1] - w[0], w[1]))
                .collect::<Vec<_>>()
                .windows(4)
                .map(|w| {
                    let mut k = (w[0].0 + 10) << 5;
                    k |= w[1].0 + 10;
                    k <<= 5;
                    k |= w[2].0 + 10;
                    k <<= 5;
                    k |= w[3].0 + 10;
                    (k, w[3].1)
                })
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
