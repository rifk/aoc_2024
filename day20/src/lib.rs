use anyhow::{anyhow, bail, Result};

#[derive(Clone, Debug, PartialEq)]
enum Map {
    Track(u32),
    End(u32),
    Start,
    Wall,
}

fn parse_input(input: &str) -> Result<Vec<Vec<Map>>> {
    let mut map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    Ok(match c {
                        'S' => Map::Start,
                        'E' => Map::End(0),
                        '.' => Map::Track(0),
                        '#' => Map::Wall,
                        _ => bail!("unexpected char {c}"),
                    })
                })
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<Vec<_>>>>()?;
    let start = map
        .iter()
        .enumerate()
        .flat_map(|(i, l)| l.iter().enumerate().map(move |(j, m)| ((i, j), m)))
        .find_map(|(pos, m)| if *m == Map::Start { Some(pos) } else { None })
        .ok_or_else(|| anyhow!("no start found"))?;
    let mut pos = start;
    let mut nano = 1;
    while !matches!(map[pos.0][pos.1], Map::End(_)) {
        for (i, j) in [
            (pos.0 + 1, pos.1),
            (pos.0 - 1, pos.1),
            (pos.0, pos.1 + 1),
            (pos.0, pos.1 - 1),
        ] {
            if map[i][j] == Map::Track(0) {
                map[i][j] = Map::Track(nano);
                pos = (i, j);
                break;
            } else if map[i][j] == Map::End(0) {
                map[i][j] = Map::End(nano);
                pos = (i, j);
                break;
            }
        }
        nano += 1;
    }
    Ok(map)
}

pub fn solve_one(input: &str) -> Result<String> {
    let map = parse_input(input)?;
    Ok(map
        .iter()
        .enumerate()
        .flat_map(|(i, l)| l.iter().enumerate().map(move |(j, m)| ((i, j), m)))
        .filter_map(|((i, j), m)| {
            match m {
                Map::Start => Some(0),
                Map::Track(nano) => Some(*nano),
                _ => None,
            }
            .map(|start_time| {
                vec![
                    i.checked_add(2).filter(|&i| i < map.len()).map(|i| (i, j)),
                    i.checked_sub(2).map(|i| (i, j)),
                    j.checked_add(2)
                        .filter(|&j| j < map[i].len())
                        .map(|j| (i, j)),
                    j.checked_sub(2).map(|j| (i, j)),
                ]
                .into_iter()
                .flatten()
                .flat_map(|(i, j)| match map[i][j] {
                    Map::Track(end_time) => end_time.checked_sub(start_time),
                    Map::End(end_time) => Some(end_time - start_time),
                    _ => None,
                })
                .collect::<Vec<_>>()
            })
        })
        .flatten()
        .filter(|&saved| saved - 2 >= 100)
        .count()
        .to_string())
}

fn pos_after_20(i: usize, j: usize, max_i: usize, max_j: usize) -> Vec<(usize, usize)> {
    (0..=20)
        .flat_map(|d_i| {
            let d_j = 20 - d_i;
            if d_i == 0 {
                vec![Some(i)]
            } else {
                vec![
                    i.checked_add(d_i).filter(|&i| i < max_i),
                    i.checked_sub(d_i),
                ]
            }
            .into_iter()
            .flatten()
            .map(|i| {
                (0..=d_j).flat_map(move |d_j| {
                    if d_j == 0 {
                        vec![Some(j)]
                    } else {
                        vec![
                            j.checked_add(d_j).filter(|&j| j < max_j),
                            j.checked_sub(d_j),
                        ]
                    }
                    .into_iter()
                    .flatten()
                    .map(|j| (i, j))
                    .collect::<Vec<_>>()
                })
            })
            .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

pub fn solve_two(input: &str) -> Result<String> {
    let map = parse_input(input)?;
    Ok(map
        .iter()
        .enumerate()
        .flat_map(|(i, l)| l.iter().enumerate().map(move |(j, m)| ((i, j), m)))
        .filter_map(|((i, j), m)| {
            match m {
                Map::Start => Some(0),
                Map::Track(nano) => Some(*nano),
                _ => None,
            }
            .map(|start_time| {
                pos_after_20(i, j, map.len(), map[i].len())
                    .into_iter()
                    .flat_map(|(to_i, to_j)| {
                        match map[to_i][to_j] {
                            Map::Track(end_time) => end_time.checked_sub(start_time),
                            Map::End(end_time) => Some(end_time - start_time),
                            _ => None,
                        }
                        .and_then(|saved| {
                            saved.checked_sub((i.abs_diff(to_i) + j.abs_diff(to_j)) as u32)
                        })
                    })
                    .collect::<Vec<_>>()
            })
        })
        .flatten()
        .filter(|&saved| saved >= 100)
        .count()
        .to_string())
}
