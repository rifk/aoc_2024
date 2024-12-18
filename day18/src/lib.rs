use std::collections::HashSet;

use anyhow::{anyhow, Result};

const START: (u8, u8) = (0, 0);
const END: (u8, u8) = (70, 70);

fn parse_input(input: &str) -> Result<Vec<(u8, u8)>> {
    input
        .lines()
        .map(|line| {
            line.split_once(',')
                .ok_or_else(|| anyhow!("missing ,"))
                .and_then(|(i, j)| Ok((i.parse::<u8>()?, j.parse::<u8>()?)))
        })
        .collect()
}

pub fn solve_one(input: &str) -> Result<String> {
    let fallen_bytes = parse_input(input)?
        .into_iter()
        .take(1024)
        .collect::<HashSet<_>>();

    let mut visited = HashSet::<(u8, u8)>::new();
    visited.insert(START);

    let mut cur = HashSet::<(u8, u8)>::new();
    cur.insert(START);

    let mut steps = 0;

    let mut end_steps = None;
    while end_steps.is_none() && !cur.is_empty() {
        steps += 1;
        let mut next = HashSet::new();
        cur.iter().for_each(|pos| {
            vec![
                pos.0
                    .checked_add(1)
                    .filter(|&i| i <= END.0)
                    .map(|i| (i, pos.1)),
                pos.0.checked_sub(1).map(|i| (i, pos.1)),
                pos.1
                    .checked_add(1)
                    .filter(|&j| j <= END.1)
                    .map(|j| (pos.0, j)),
                pos.1.checked_sub(1).map(|j| (pos.0, j)),
            ]
            .into_iter()
            .flatten()
            .for_each(|next_pos| {
                if next_pos == END {
                    end_steps = Some(steps);
                } else if !fallen_bytes.contains(&next_pos) && !visited.contains(&next_pos) {
                    next.insert(next_pos);
                    visited.insert(next_pos);
                }
            });
        });
        std::mem::swap(&mut cur, &mut next);
    }

    Ok(end_steps.unwrap().to_string())
}

fn has_path(fallen_bytes: &HashSet<&(u8, u8)>) -> bool {
    let mut visited = HashSet::<(u8, u8)>::new();
    visited.insert(START);

    let mut cur = HashSet::<(u8, u8)>::new();
    cur.insert(START);

    while !visited.contains(&END) && !cur.is_empty() {
        let mut next = HashSet::new();
        cur.iter().for_each(|pos| {
            vec![
                pos.0
                    .checked_add(1)
                    .filter(|&i| i <= END.0)
                    .map(|i| (i, pos.1)),
                pos.0.checked_sub(1).map(|i| (i, pos.1)),
                pos.1
                    .checked_add(1)
                    .filter(|&j| j <= END.1)
                    .map(|j| (pos.0, j)),
                pos.1.checked_sub(1).map(|j| (pos.0, j)),
            ]
            .into_iter()
            .flatten()
            .for_each(|next_pos| {
                if !fallen_bytes.contains(&next_pos) && !visited.contains(&next_pos) {
                    next.insert(next_pos);
                    visited.insert(next_pos);
                }
            });
        });
        std::mem::swap(&mut cur, &mut next);
    }
    visited.contains(&END)
}

pub fn solve_two(input: &str) -> Result<String> {
    let input = parse_input(input)?;

    let p = (0..input.len())
        .collect::<Vec<_>>()
        .partition_point(|i| has_path(&input.iter().take(i + 1).collect()));

    Ok(format!("{},{}", input[p].0, input[p].1))
}
