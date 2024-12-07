use anyhow::{anyhow, Result};

pub fn solve_one(input: &str) -> Result<String> {
    let lines = parse_input(input)?;
    Ok(lines
        .into_iter()
        .filter(|(total, list)| {
            can_equal(
                *total,
                list,
                1,
                list[0],
                list.iter().skip(1).filter(|&v| *v != 1).sum::<u64>(),
            )
        })
        .map(|(total, _)| total)
        .sum::<u64>()
        .to_string())
}

fn parse_input(input: &str) -> Result<Vec<(u64, Vec<u64>)>> {
    input
        .lines()
        .map(|line| {
            let (total, list) = line
                .split_once(": ")
                .ok_or_else(|| anyhow!("missing ': ' in line"))?;
            Ok((
                total.parse()?,
                list.split_whitespace()
                    .map(|v| Ok(v.parse()?))
                    .collect::<Result<Vec<_>>>()?,
            ))
        })
        .collect()
}

fn can_equal(total: u64, list: &[u64], pos: usize, cur: u64, rem: u64) -> bool {
    if pos >= list.len() {
        return total == cur;
    } else if total == cur + rem {
        return true;
    } else if total < cur + rem {
        return false;
    }

    can_equal(
        total,
        list,
        pos + 1,
        cur + list[pos],
        if list[pos] == 1 { rem } else { rem - list[pos] },
    ) || can_equal(
        total,
        list,
        pos + 1,
        cur * list[pos],
        if list[pos] == 1 { rem } else { rem - list[pos] },
    )
}

pub fn solve_two(input: &str) -> Result<String> {
    let lines = parse_input(input)?;
    Ok(lines
        .into_iter()
        .filter(|(total, list)| {
            can_equal_with_cat(
                *total,
                list,
                1,
                list[0],
                list.iter().skip(1).filter(|&v| *v != 1).sum::<u64>(),
            )
        })
        .map(|(total, _)| total)
        .sum::<u64>()
        .to_string())
}

fn can_equal_with_cat(total: u64, list: &[u64], pos: usize, cur: u64, rem: u64) -> bool {
    if pos >= list.len() {
        return total == cur;
    } else if total == cur + rem {
        return true;
    } else if total < cur + rem {
        return false;
    }

    can_equal_with_cat(
        total,
        list,
        pos + 1,
        cur + list[pos],
        if list[pos] == 1 { rem } else { rem - list[pos] },
    ) || can_equal_with_cat(
        total,
        list,
        pos + 1,
        cur * list[pos],
        if list[pos] == 1 { rem } else { rem - list[pos] },
    ) || can_equal_with_cat(
        total,
        list,
        pos + 1,
        cat(cur, list[pos]),
        if list[pos] == 1 { rem } else { rem - list[pos] },
    )
}

fn cat(l: u64, r: u64) -> u64 {
    let mut tens = 10;
    while (r / tens) > 0 {
        tens *= 10;
    }
    (l * tens) + r
}
