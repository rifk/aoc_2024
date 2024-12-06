use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};

struct ParsedInput {
    rules: HashMap<usize, HashSet<usize>>,
    updates: Vec<Vec<usize>>,
}

fn parse_input(input: &str) -> Result<ParsedInput> {
    let (rules, updates) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("missing empty line"))?;
    Ok(ParsedInput {
        rules: rules
            .lines()
            .map(|line| {
                let (l, r) = line.split_once('|').ok_or_else(|| anyhow!("missing |"))?;
                Ok((l.parse::<usize>()?, r.parse::<usize>()?))
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .fold(HashMap::new(), |mut map, (l, r)| {
                map.entry(l)
                    .and_modify(|v: &mut HashSet<usize>| {
                        v.insert(r);
                    })
                    .or_insert_with(|| {
                        let mut set = HashSet::new();
                        set.insert(r);
                        set
                    });
                map
            }),
        updates: updates
            .lines()
            .map(|line| {
                line.split(',')
                    .map(|v| Ok(v.parse::<usize>()?))
                    .collect::<Result<Vec<usize>>>()
            })
            .collect::<Result<Vec<Vec<usize>>>>()?,
    })
}

pub fn solve_one(input: &str) -> Result<String> {
    let ParsedInput { rules, updates } = parse_input(input)?;

    Ok(updates
        .into_iter()
        .filter_map(|update| {
            let mut seen = HashSet::new();
            if update.iter().any(|v| {
                if rules
                    .get(v)
                    .map(|after| seen.intersection(after).count() != 0)
                    .unwrap_or(false)
                {
                    true
                } else {
                    seen.insert(*v);
                    false
                }
            }) {
                None
            } else {
                Some(update[update.len() / 2])
            }
        })
        .sum::<usize>()
        .to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    let ParsedInput { rules, updates } = parse_input(input)?;

    Ok(updates
        .into_iter()
        .filter_map(|update| {
            let mut seen = HashSet::new();
            update.iter().find(|&v| {
                if rules
                    .get(v)
                    .map(|after| seen.intersection(after).count() != 0)
                    .unwrap_or(false)
                {
                    true
                } else {
                    seen.insert(*v);
                    false
                }
            })?;
            let mut update = update.clone();
            (0..update.len() - 1).for_each(|i| {
                (i + 1..update.len()).for_each(|j| {
                    if rules
                        .get(&update[j])
                        .map(|after| after.contains(&update[i]))
                        .unwrap_or(false)
                    {
                        update.swap(i, j);
                    }
                });
            });
            Some(update[update.len() / 2])
        })
        .sum::<usize>()
        .to_string())
}
