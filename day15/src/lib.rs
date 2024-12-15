use std::collections::HashSet;

use anyhow::{anyhow, bail, Result};

#[derive(Clone, Debug, Eq, PartialEq)]
enum Map {
    Wall,
    Box,
    Robot,
    Empty,
}
#[derive(Clone, Debug, Eq, PartialEq)]
enum Dir {
    U,
    D,
    L,
    R,
}
impl Dir {
    fn next(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Dir::U => (pos.0 - 1, pos.1),
            Dir::D => (pos.0 + 1, pos.1),
            Dir::L => (pos.0, pos.1 - 1),
            Dir::R => (pos.0, pos.1 + 1),
        }
    }
    fn op(&self) -> Self {
        match self {
            Dir::U => Dir::D,
            Dir::D => Dir::U,
            Dir::L => Dir::R,
            Dir::R => Dir::L,
        }
    }
}
struct Input {
    map: Vec<Vec<Map>>,
    robot_pos: (usize, usize),
    movement: Vec<Dir>,
}
fn parse_input(input: &str) -> Result<Input> {
    let (map, movement) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("missing empty line"))?;
    let map = map
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Ok(Map::Empty),
                    '#' => Ok(Map::Wall),
                    'O' => Ok(Map::Box),
                    '@' => Ok(Map::Robot),
                    _ => bail!("unexpected map char {c}"),
                })
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<Vec<Map>>>>()?;
    let robot_pos = {
        let mut pos = None;
        for (i, l) in map.iter().enumerate() {
            for (j, m) in l.iter().enumerate() {
                if *m == Map::Robot {
                    if pos.is_some() {
                        bail!("multiple robots in map");
                    }
                    pos = Some((i, j));
                }
            }
        }
        pos.ok_or_else(|| anyhow!("no robot in map"))?
    };
    let movement = movement
        .lines()
        .flat_map(|l| l.chars())
        .map(|c| match c {
            '^' => Ok(Dir::U),
            'v' => Ok(Dir::D),
            '<' => Ok(Dir::L),
            '>' => Ok(Dir::R),
            _ => bail!("unexpected dir char {c}"),
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(Input {
        map,
        robot_pos,
        movement,
    })
}

pub fn solve_one(input: &str) -> Result<String> {
    let Input {
        mut map,
        mut robot_pos,
        movement,
    } = parse_input(input)?;

    for dir in movement {
        move_robot(dir, &mut map, &mut robot_pos)?;
    }

    Ok(map
        .into_iter()
        .enumerate()
        .flat_map(|(i, l)| l.into_iter().enumerate().map(move |(j, m)| (i, j, m)))
        .map(|(i, j, m)| if m == Map::Box { (100 * i) + j } else { 0 })
        .sum::<usize>()
        .to_string())
}

fn move_robot(dir: Dir, map: &mut [Vec<Map>], robot_pos: &mut (usize, usize)) -> Result<()> {
    let mut empty = None;
    let mut next = dir.next(*robot_pos);
    while empty.is_none() && map[next.0][next.1] != Map::Wall {
        if map[next.0][next.1] == Map::Empty {
            empty = Some(next);
        }
        next = dir.next(next);
    }
    if let Some(mut empty) = empty {
        let op = dir.op();
        let mut robot_moved = false;
        let mut next = op.next(empty);
        while !robot_moved {
            match map[next.0][next.1] {
                Map::Wall => bail!("unexpected wall {next:?}"),
                Map::Box => {
                    map[empty.0][empty.1] = Map::Box;
                    map[next.0][next.1] = Map::Empty;
                    empty = op.next(empty);
                }
                Map::Robot => {
                    map[empty.0][empty.1] = Map::Robot;
                    map[next.0][next.1] = Map::Empty;
                    robot_moved = true;
                    *robot_pos = empty;
                }
                Map::Empty => {}
            }
            next = op.next(next);
        }
    }
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum WideMap {
    Wall,
    BoxL,
    BoxR,
    Robot,
    Empty,
}
pub fn solve_two(input: &str) -> Result<String> {
    let Input { map, movement, .. } = parse_input(input)?;
    let mut map = map
        .into_iter()
        .map(|l| {
            l.into_iter()
                .flat_map(|m| match m {
                    Map::Wall => vec![WideMap::Wall, WideMap::Wall],
                    Map::Box => vec![WideMap::BoxL, WideMap::BoxR],
                    Map::Robot => vec![WideMap::Robot, WideMap::Empty],
                    Map::Empty => vec![WideMap::Empty, WideMap::Empty],
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();
    let mut robot_pos = {
        let mut pos = None;
        for (i, l) in map.iter().enumerate() {
            for (j, m) in l.iter().enumerate() {
                if *m == WideMap::Robot {
                    if pos.is_some() {
                        bail!("multiple robots in map");
                    }
                    pos = Some((i, j));
                }
            }
        }
        pos.ok_or_else(|| anyhow!("no robot in map"))?
    };

    for dir in movement {
        move_wide_robot(dir, &mut map, &mut robot_pos)?;
    }

    Ok(map
        .into_iter()
        .enumerate()
        .flat_map(|(i, l)| l.into_iter().enumerate().map(move |(j, m)| (i, j, m)))
        .map(
            |(i, j, m)| {
                if m == WideMap::BoxL {
                    (100 * i) + j
                } else {
                    0
                }
            },
        )
        .sum::<usize>()
        .to_string())
}

fn move_wide_robot(
    dir: Dir,
    map: &mut Vec<Vec<WideMap>>,
    robot_pos: &mut (usize, usize),
) -> Result<()> {
    let mut new_map = map.clone();
    let mut from = HashSet::new();
    let mut to = HashSet::new();
    if move_wide(&dir, map, &mut new_map, *robot_pos, &mut from, &mut to)? {
        from.difference(&to)
            .for_each(|&(i, j)| new_map[i][j] = WideMap::Empty);
        std::mem::swap(map, &mut new_map);
        *robot_pos = dir.next(*robot_pos);
    }
    Ok(())
}

fn move_wide(
    dir: &Dir,
    map: &[Vec<WideMap>],
    new_map: &mut [Vec<WideMap>],
    pos: (usize, usize),
    from: &mut HashSet<(usize, usize)>,
    to: &mut HashSet<(usize, usize)>,
) -> Result<bool> {
    let next = dir.next(pos);
    from.insert(pos);
    to.insert(next);
    new_map[next.0][next.1] = map[pos.0][pos.1].clone();
    Ok(match map[next.0][next.1] {
        WideMap::Wall => false,
        WideMap::BoxL => {
            move_wide(dir, map, new_map, next, from, to)?
                && (*dir == Dir::L
                    || *dir == Dir::R
                    || move_wide(dir, map, new_map, Dir::R.next(next), from, to)?)
        }
        WideMap::BoxR => {
            move_wide(dir, map, new_map, next, from, to)?
                && (*dir == Dir::L
                    || *dir == Dir::R
                    || move_wide(dir, map, new_map, Dir::L.next(next), from, to)?)
        }
        WideMap::Robot => bail!("unexpected robot"),
        WideMap::Empty => true,
    })
}
