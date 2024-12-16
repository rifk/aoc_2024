use std::collections::{BTreeMap, HashMap, HashSet};

use anyhow::{anyhow, bail, Result};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum Dir {
    N,
    S,
    E,
    W,
}
impl Dir {
    fn turn_cw(&self) -> Self {
        match self {
            Dir::N => Dir::E,
            Dir::S => Dir::W,
            Dir::E => Dir::S,
            Dir::W => Dir::N,
        }
    }
    fn turn_ccw(&self) -> Self {
        match self {
            Dir::N => Dir::W,
            Dir::S => Dir::E,
            Dir::E => Dir::N,
            Dir::W => Dir::S,
        }
    }
    fn step(&self, i: usize, j: usize) -> (usize, usize) {
        match self {
            Dir::N => (i - 1, j),
            Dir::S => (i + 1, j),
            Dir::E => (i, j + 1),
            Dir::W => (i, j - 1),
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
enum Maze {
    Wall,
    Walk,
    End,
}

struct Input {
    maze: Vec<Vec<Maze>>,
    start: (usize, usize, Dir),
}

fn parse_input(input: &str) -> Result<Input> {
    let maze = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    Ok(match c {
                        '#' => Maze::Wall,
                        '.' | 'S' => Maze::Walk,
                        'E' => Maze::End,
                        _ => bail!("unexpected char {c}"),
                    })
                })
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<Vec<_>>>>()?;
    let start = input
        .lines()
        .enumerate()
        .find_map(|(i, line)| {
            line.chars()
                .enumerate()
                .find_map(|(j, char)| if char == 'S' { Some((i, j)) } else { None })
        })
        .map(|(i, j)| (i, j, Dir::E))
        .ok_or_else(|| anyhow!("missing S"))?;
    Ok(Input { maze, start })
}

pub fn solve_one(input: &str) -> Result<String> {
    let Input { maze, start } = parse_input(input)?;

    let mut routes = BTreeMap::new();
    routes.insert(0, {
        let mut set = HashSet::new();
        set.insert(start);
        set
    });

    let mut end = None;
    while end.is_none() {
        let (score, set) = routes.pop_first().ok_or_else(|| anyhow!("empty routes"))?;
        for (i, j, dir) in set {
            if maze[i][j] == Maze::End {
                end = Some(score);
            }

            vec![
                (score + 1001, dir.turn_cw()),
                (score + 1001, dir.turn_ccw()),
                (score + 1, dir),
            ]
            .into_iter()
            .for_each(|(score, dir)| {
                let (i, j) = dir.step(i, j);
                if maze[i][j] != Maze::Wall {
                    routes
                        .entry(score)
                        .and_modify(|set| {
                            set.insert((i, j, dir.clone()));
                        })
                        .or_insert_with(|| {
                            let mut set = HashSet::new();
                            set.insert((i, j, dir));
                            set
                        });
                }
            });
        }
    }

    Ok(end.unwrap().to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    let Input { maze, start } = parse_input(input)?;

    let mut routes = BTreeMap::new();
    routes.insert(0, {
        let mut map = HashMap::new();
        let mut steps = HashSet::new();
        steps.insert((start.0, start.1));
        map.insert(start, steps);
        map
    });

    let mut end_steps = HashSet::new();
    while end_steps.is_empty() {
        let (score, map) = routes.pop_first().ok_or_else(|| anyhow!("empty routes"))?;
        for ((i, j, dir), steps) in map {
            if maze[i][j] == Maze::End {
                steps.iter().for_each(|&s| {
                    end_steps.insert(s);
                });
                continue;
            }

            vec![
                (score + 1001, dir.turn_cw()),
                (score + 1001, dir.turn_ccw()),
                (score + 1, dir),
            ]
            .into_iter()
            .for_each(|(score, dir)| {
                let (i, j) = dir.step(i, j);
                if maze[i][j] != Maze::Wall {
                    routes
                        .entry(score)
                        .and_modify(|map| {
                            map.entry((i, j, dir.clone()))
                                .and_modify(|cur_steps| {
                                    cur_steps.insert((i, j));
                                    steps.iter().for_each(|&s| {
                                        cur_steps.insert(s);
                                    });
                                })
                                .or_insert_with(|| {
                                    let mut steps = steps.clone();
                                    steps.insert((i, j));
                                    steps
                                });
                        })
                        .or_insert_with(|| {
                            let mut set = HashMap::new();
                            let mut steps = steps.clone();
                            steps.insert((i, j));
                            set.insert((i, j, dir), steps);
                            set
                        });
                }
            });
        }
    }

    Ok(end_steps.len().to_string())
}
