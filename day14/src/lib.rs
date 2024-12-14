use std::collections::HashSet;

use anyhow::{anyhow, Result};

const ROOM: (i32, i32) = (101, 103);

type Robot = ((i32, i32), (i32, i32));

fn parse_input(input: &str) -> Result<Vec<Robot>> {
    input
        .lines()
        .map(|s| {
            let s = s.trim_start_matches("p=");
            let (p, v) = s.split_once(" v=").ok_or_else(|| anyhow!("no v="))?;
            let p = p.split_once(',').ok_or_else(|| anyhow!("missing p ','"))?;
            let v = v.split_once(',').ok_or_else(|| anyhow!("missing v ','"))?;
            Ok((
                (p.0.parse::<i32>()?, p.1.parse::<i32>()?),
                (v.0.parse::<i32>()?, v.1.parse::<i32>()?),
            ))
        })
        .collect::<Result<Vec<_>>>()
}

fn positions(robots: &[Robot], secs: i32) -> Vec<(i32, i32)> {
    robots
        .iter()
        .map(|(p, v)| {
            (
                (p.0 + (secs * v.0)).rem_euclid(ROOM.0),
                (p.1 + (secs * v.1)).rem_euclid(ROOM.1),
            )
        })
        .collect()
}

pub fn solve_one(input: &str) -> Result<String> {
    let robots = parse_input(input)?;
    Ok(positions(&robots, 100)
        .into_iter()
        .filter_map(
            |pos| match (pos.0.cmp(&(ROOM.0 / 2)), pos.1.cmp(&(ROOM.1 / 2))) {
                (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => Some(0),
                (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => Some(1),
                (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => Some(2),
                (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => Some(3),
                (_, _) => None,
            },
        )
        .fold(vec![0; 4], |mut vec: Vec<u32>, quad| {
            vec[quad] += 1;
            vec
        })
        .into_iter()
        .product::<u32>()
        .to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    let robots = parse_input(input)?;

    let mut secs = 0;
    let mut tree = false;
    while !tree && secs < 50000 {
        secs += 1;
        let pos = positions(&robots, secs).into_iter().collect::<HashSet<_>>();

        let isolated = pos
            .iter()
            .filter(|p| {
                !pos.contains(&(p.0 + 1, p.1))
                    && !pos.contains(&(p.0 - 1, p.1))
                    && !pos.contains(&(p.0, p.1 + 1))
                    && !pos.contains(&(p.0, p.1 - 1))
            })
            .count();

        if isolated < pos.len() / 2 {
            tree = true;
            // print pos
            //let mut r = vec![vec![' '; ROOM.0 as usize]; ROOM.1 as usize];
            //pos.iter()
            //    .for_each(|&(i, j)| r[j as usize][i as usize] = '#');
            //r.iter().for_each(|l| {
            //    l.iter().for_each(|c| print!("{c}"));
            //    println!("");
            //});
        }
    }
    if !tree {
        anyhow::bail!("couldnt find tree after {secs} secs");
    }
    Ok(secs.to_string())
}
