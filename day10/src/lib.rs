use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Result};

pub fn solve_one(input: &str) -> Result<String> {
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).ok_or_else(|| anyhow!("not a digit")))
                .collect::<Result<Vec<u32>>>()
        })
        .collect::<Result<Vec<Vec<u32>>>>()?;
    let (max_i, max_j) = (map.len() - 1, map[0].len() - 1);
    Ok((0..=max_i)
        .flat_map(|i| (0..=max_j).map(move |j| (i, j)))
        .map(|(i, j)| {
            let mut steps = HashSet::new();
            let mut step = 0;
            if map[i][j] == step {
                steps.insert((i, j));
            }
            while step < 9 && !steps.is_empty() {
                let mut next = HashSet::new();
                step += 1;
                steps.iter().for_each(|&(i, j)| {
                    vec![
                        i.checked_sub(1).map(|i| (i, j)),
                        i.checked_add(1).filter(|&i| i <= max_i).map(|i| (i, j)),
                        j.checked_sub(1).map(|j| (i, j)),
                        j.checked_add(1).filter(|&j| j <= max_j).map(|j| (i, j)),
                    ]
                    .into_iter()
                    .flatten()
                    .for_each(|(i, j): (usize, usize)| {
                        if map[i][j] == step {
                            next.insert((i, j));
                        }
                    })
                });
                std::mem::swap(&mut steps, &mut next);
            }
            steps.len()
        })
        .sum::<usize>()
        .to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).ok_or_else(|| anyhow!("not a digit")))
                .collect::<Result<Vec<u32>>>()
        })
        .collect::<Result<Vec<Vec<u32>>>>()?;
    let (max_i, max_j) = (map.len() - 1, map[0].len() - 1);
    Ok((0..=max_i)
        .flat_map(|i| (0..=max_j).map(move |j| (i, j)))
        .map(|(i, j)| {
            let mut steps = HashMap::new();
            let mut step = 0;
            if map[i][j] == step {
                steps.insert((i, j), 1);
            }
            while step < 9 && !steps.is_empty() {
                let mut next = HashMap::new();
                step += 1;
                steps.iter().for_each(|(&(i, j), &count)| {
                    vec![
                        i.checked_sub(1).map(|i| (i, j)),
                        i.checked_add(1).filter(|&i| i <= max_i).map(|i| (i, j)),
                        j.checked_sub(1).map(|j| (i, j)),
                        j.checked_add(1).filter(|&j| j <= max_j).map(|j| (i, j)),
                    ]
                    .into_iter()
                    .flatten()
                    .for_each(|(i, j): (usize, usize)| {
                        if map[i][j] == step {
                            next.entry((i, j))
                                .and_modify(|c| *c += count)
                                .or_insert(count);
                        }
                    })
                });
                std::mem::swap(&mut steps, &mut next);
            }
            steps.values().sum::<usize>()
        })
        .sum::<usize>()
        .to_string())
}
