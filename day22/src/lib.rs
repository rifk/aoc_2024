use std::collections::{HashMap, HashSet};

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
    let secrets = input
        .lines()
        .map(|v| Ok(v.parse::<i64>()?))
        .collect::<Result<Vec<_>>>()?;

    Ok(secrets
        .into_iter()
        .map(|mut s| {
            for _ in 0..2000 {
                s = next_secret(s);
            }
            s
        })
        .sum::<i64>()
        .to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    let monkeys = input
        .lines()
        .map(|v| Ok(v.parse::<i64>()?))
        .collect::<Result<Vec<_>>>()?;

    let mut diffs_sum = HashMap::new();
    for s in monkeys {
        let secrets = {
            let mut s = s;
            let mut v = vec![s];
            for _ in 0..2000 {
                s = next_secret(s);
                v.push(s);
            }
            v
        };
        let diffs = secrets
            .windows(2)
            .map(|w| (w[1] % 10 - w[0] % 10, w[1] % 10))
            .collect::<Vec<_>>();
        let mut seen_four_diffs = HashSet::new();
        diffs.windows(4).for_each(|w| {
            let four_diffs = (w[0].0, w[1].0, w[2].0, w[3].0);
            if seen_four_diffs.insert(four_diffs) {
                diffs_sum
                    .entry(four_diffs)
                    .and_modify(|sum| *sum += w[3].1)
                    .or_insert(w[3].1);
            }
        });
    }

    Ok(diffs_sum.values().max().unwrap().to_string())
}
