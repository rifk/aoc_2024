use std::collections::{HashMap, HashSet};

use anyhow::Result;

pub fn solve_one(input: &str) -> Result<String> {
    let plots = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let max_i_j = (plots.len(), plots[0].len());
    let mut seen = vec![vec![false; max_i_j.1]; max_i_j.0];

    Ok((0..max_i_j.0)
        .flat_map(|i| (0..max_i_j.1).map(move |j| (i, j)))
        .filter_map(|(i, j)| {
            let region = area_perimeter(&plots, &mut seen, max_i_j, (i, j), plots[i][j]);
            if region.0 == 0 {
                None
            } else {
                Some(region.0 * region.1)
            }
        })
        .sum::<u32>()
        .to_string())
}

fn area_perimeter(
    plots: &[Vec<char>],
    seen: &mut [Vec<bool>],
    max_i_j: (usize, usize),
    i_j: (usize, usize),
    last: char,
) -> (u32, u32) {
    if last != plots[i_j.0][i_j.1] {
        return (0, 1);
    }
    if seen[i_j.0][i_j.1] {
        return (0, 0);
    }
    seen[i_j.0][i_j.1] = true;
    vec![
        i_j.0.checked_sub(1).map(|i| (i, i_j.1)),
        i_j.0
            .checked_add(1)
            .filter(|&i| i < max_i_j.0)
            .map(|i| (i, i_j.1)),
        i_j.1.checked_sub(1).map(|j| (i_j.0, j)),
        i_j.1
            .checked_add(1)
            .filter(|&j| j < max_i_j.1)
            .map(|j| (i_j.0, j)),
    ]
    .into_iter()
    .map(|next| match next {
        Some(i_j) => area_perimeter(plots, seen, max_i_j, i_j, last),
        None => (0, 1),
    })
    .fold((1, 0), |acc, ap| (acc.0 + ap.0, acc.1 + ap.1))
}

pub fn solve_two(input: &str) -> Result<String> {
    let plots = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let max_i_j = (plots.len(), plots[0].len());
    let mut seen = vec![vec![false; max_i_j.1]; max_i_j.0];

    Ok((0..max_i_j.0)
        .flat_map(|i| (0..max_i_j.1).map(move |j| (i, j)))
        .filter_map(|(i, j)| {
            let region = area_edges(&plots, &mut seen, max_i_j, (i, j), (i, j), plots[i][j]);
            if region.0 == 0 {
                None
            } else {
                Some(
                    region.0
                        * region
                            .1
                            .into_iter()
                            .map(|(i, j, dir)| match dir {
                                Dir::N | Dir::S => ((dir, i), j),
                                Dir::E | Dir::W => ((dir, j), i),
                            })
                            .fold(HashMap::new(), |mut map, (k, v)| {
                                map.entry(k)
                                    .and_modify(|vec: &mut Vec<usize>| vec.push(v))
                                    .or_insert_with(|| vec![v]);
                                map
                            })
                            .into_values()
                            .map(|mut v| {
                                v.sort();
                                1 + v.windows(2).filter(|w| w[0].abs_diff(w[1]) > 1).count() as u32
                            })
                            .sum::<u32>(),
                )
            }
        })
        .sum::<u32>()
        .to_string())
}

#[derive(Eq, PartialEq, Hash)]
enum Dir {
    N,
    S,
    E,
    W,
}
impl Dir {
    fn of(from: (usize, usize), to: (usize, usize)) -> Self {
        match from.0.cmp(&to.0) {
            std::cmp::Ordering::Less => Self::S,
            std::cmp::Ordering::Equal => {
                if from.1 < to.1 {
                    Self::E
                } else {
                    Self::W
                }
            }
            std::cmp::Ordering::Greater => Self::N,
        }
    }
}
fn area_edges(
    plots: &[Vec<char>],
    seen: &mut [Vec<bool>],
    max_i_j: (usize, usize),
    i_j: (usize, usize),
    last_i_j: (usize, usize),
    last: char,
) -> (u32, HashSet<(usize, usize, Dir)>) {
    if last != plots[i_j.0][i_j.1] {
        let mut edge = HashSet::new();
        edge.insert((last_i_j.0, last_i_j.1, Dir::of(last_i_j, i_j)));
        return (0, edge);
    }
    if seen[i_j.0][i_j.1] {
        return (0, HashSet::new());
    }
    seen[i_j.0][i_j.1] = true;
    vec![
        i_j.0
            .checked_sub(1)
            .map(|i| (Some((i, i_j.1)), Dir::N))
            .unwrap_or_else(|| (None, Dir::N)),
        i_j.0
            .checked_add(1)
            .filter(|&i| i < max_i_j.0)
            .map(|i| (Some((i, i_j.1)), Dir::S))
            .unwrap_or_else(|| (None, Dir::S)),
        i_j.1
            .checked_sub(1)
            .map(|j| (Some((i_j.0, j)), Dir::W))
            .unwrap_or_else(|| (None, Dir::W)),
        i_j.1
            .checked_add(1)
            .filter(|&j| j < max_i_j.1)
            .map(|j| (Some((i_j.0, j)), Dir::E))
            .unwrap_or_else(|| (None, Dir::E)),
    ]
    .into_iter()
    .map(|(next, dir)| match next {
        Some(next_i_j) => area_edges(plots, seen, max_i_j, next_i_j, i_j, last),
        None => {
            let mut edge = HashSet::new();
            edge.insert((i_j.0, i_j.1, dir));
            (0, edge)
        }
    })
    .fold((1, HashSet::new()), |(area, mut edges), ae| {
        ae.1.into_iter().for_each(|e| {
            edges.insert(e);
        });
        (area + ae.0, edges)
    })
}
