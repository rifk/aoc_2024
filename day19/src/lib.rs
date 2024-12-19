use std::collections::HashMap;

use anyhow::{anyhow, bail, Result};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Colour {
    White,
    Blue,
    Black,
    Red,
    Green,
}
impl Colour {
    fn new(c: char) -> Result<Self> {
        Ok(match c {
            'w' => Self::White,
            'u' => Self::Blue,
            'b' => Self::Black,
            'r' => Self::Red,
            'g' => Self::Green,
            _ => bail!("unexpected '{c}'"),
        })
    }
}

type Input = (Vec<Vec<Colour>>, Vec<Vec<Colour>>);
fn parse_input(input: &str) -> Result<Input> {
    let (available, desired) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("no empty line"))?;
    Ok((
        available
            .split(", ")
            .map(|towel| towel.chars().map(Colour::new).collect::<Result<Vec<_>>>())
            .collect::<Result<Vec<Vec<_>>>>()?,
        desired
            .lines()
            .map(|towel| towel.chars().map(Colour::new).collect::<Result<Vec<_>>>())
            .collect::<Result<Vec<Vec<_>>>>()?,
    ))
}

fn is_possible(desired: &[Colour], available: &[Vec<Colour>]) -> bool {
    if desired.is_empty() {
        return true;
    }
    for a in available {
        if desired.starts_with(a) && is_possible(&desired[a.len()..], available) {
            return true;
        }
    }
    false
}

pub fn solve_one(input: &str) -> Result<String> {
    let (available, desired) = parse_input(input)?;

    Ok(desired
        .into_iter()
        .filter(|d| is_possible(d, &available))
        .count()
        .to_string())
}

fn count_possible(
    mem: &mut HashMap<Vec<Colour>, u64>,
    desired: &[Colour],
    available: &[Vec<Colour>],
) -> u64 {
    if desired.is_empty() {
        return 1;
    }
    if let Some(c) = mem.get(desired) {
        return *c;
    }
    let mut c = 0;
    for a in available {
        if desired.starts_with(a) {
            c += count_possible(mem, &desired[a.len()..], available)
        }
    }
    mem.insert(desired.to_vec(), c);
    c
}

pub fn solve_two(input: &str) -> Result<String> {
    let (available, desired) = parse_input(input)?;

    let mut mem = HashMap::new();

    Ok(desired
        .into_iter()
        .map(|d| count_possible(&mut mem, &d, &available))
        .sum::<u64>()
        .to_string())
}
