use std::collections::{HashMap, HashSet};

use anyhow::Result;

struct Input {
    antennas: HashMap<char, Vec<(usize, usize)>>,
    max_i_j: (usize, usize),
}
fn parse_input(input: &str) -> Input {
    Input {
        antennas: input
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .filter(|&(_, char)| char != '.')
                    .map(move |(j, char)| (i, j, char))
            })
            .fold(HashMap::new(), |mut map, (i, j, char)| {
                map.entry(char)
                    .and_modify(|vec: &mut Vec<_>| vec.push((i, j)))
                    .or_insert_with(|| vec![(i, j)]);
                map
            }),
        max_i_j: (input.lines().count(), input.lines().next().unwrap().len()),
    }
}

pub fn solve_one(input: &str) -> Result<String> {
    let Input { antennas, max_i_j } = parse_input(input);
    Ok(antennas
        .iter()
        .flat_map(|(_, antennas)| {
            (0..antennas.len() - 1)
                .flat_map(|a| (a + 1..antennas.len()).map(move |b| (a, b)))
                .flat_map(|(a, b)| {
                    let mut antinodes = Vec::new();
                    let (a_i, a_j) = antennas[a];
                    let (b_i, b_j) = antennas[b];
                    let i_diff = a_i.abs_diff(b_i);
                    let j_diff = a_j.abs_diff(b_j);
                    let (anti_a_i, anti_b_i) = if a_i < b_i {
                        (
                            a_i.checked_sub(i_diff),
                            b_i.checked_add(i_diff).filter(|&i| i < max_i_j.0),
                        )
                    } else {
                        (
                            a_i.checked_add(i_diff).filter(|&i| i < max_i_j.0),
                            b_i.checked_sub(i_diff),
                        )
                    };
                    let (anti_a_j, anti_b_j) = if a_j < b_j {
                        (
                            a_j.checked_sub(j_diff),
                            b_j.checked_add(j_diff).filter(|&j| j < max_i_j.1),
                        )
                    } else {
                        (
                            a_j.checked_add(j_diff).filter(|&j| j < max_i_j.1),
                            b_j.checked_sub(j_diff),
                        )
                    };
                    if let (Some(i), Some(j)) = (anti_a_i, anti_a_j) {
                        antinodes.push((i, j));
                    }
                    if let (Some(i), Some(j)) = (anti_b_i, anti_b_j) {
                        antinodes.push((i, j));
                    }
                    antinodes
                })
        })
        .collect::<HashSet<(usize, usize)>>()
        .len()
        .to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    let Input { antennas, max_i_j } = parse_input(input);
    Ok(antennas
        .iter()
        .flat_map(|(_, antennas)| {
            (0..antennas.len() - 1)
                .flat_map(|a| (a + 1..antennas.len()).map(move |b| (a, b)))
                .flat_map(|(a, b)| {
                    let mut antinodes = Vec::new();
                    let mut anti_a = antennas[a];
                    let mut anti_b = antennas[b];
                    let diff = (anti_a.0.abs_diff(anti_b.0), anti_a.1.abs_diff(anti_b.1));
                    let (a_v_dir, b_v_dir) = if anti_a.0 < anti_b.0 {
                        (VDir::U, VDir::D)
                    } else {
                        (VDir::D, VDir::U)
                    };
                    let (a_h_dir, b_h_dir) = if anti_a.1 < anti_b.1 {
                        (HDir::L, HDir::R)
                    } else {
                        (HDir::R, HDir::L)
                    };
                    antinodes.push(anti_a);
                    while let Some(next_a) = next_anti(anti_a, diff, (&a_v_dir, &a_h_dir), max_i_j)
                    {
                        antinodes.push(next_a);
                        anti_a = next_a;
                    }
                    antinodes.push(anti_b);
                    while let Some(next_b) = next_anti(anti_b, diff, (&b_v_dir, &b_h_dir), max_i_j)
                    {
                        antinodes.push(next_b);
                        anti_b = next_b;
                    }
                    antinodes
                })
        })
        .collect::<HashSet<(usize, usize)>>()
        .len()
        .to_string())
}

enum VDir {
    U,
    D,
}
enum HDir {
    L,
    R,
}
fn next_anti(
    cur: (usize, usize),
    diff: (usize, usize),
    dir: (&VDir, &HDir),
    max_i_j: (usize, usize),
) -> Option<(usize, usize)> {
    let i = match dir.0 {
        VDir::U => cur.0.checked_sub(diff.0)?,
        VDir::D => cur.0.checked_add(diff.0).filter(|&i| i < max_i_j.0)?,
    };
    let j = match dir.1 {
        HDir::L => cur.1.checked_sub(diff.1)?,
        HDir::R => cur.1.checked_add(diff.1).filter(|&j| j < max_i_j.1)?,
    };
    Some((i, j))
}
