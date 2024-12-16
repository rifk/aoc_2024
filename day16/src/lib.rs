use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use anyhow::{anyhow, bail, Result};

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
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

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, start.clone())));

    let mut shortest = HashMap::new();
    shortest.insert(start, 0);

    let mut visited = HashSet::new();

    let mut end = None;
    while end.is_none() {
        let (score, (i, j, dir)) = heap.pop().unwrap().0;
        if maze[i][j] == Maze::End {
            end = Some(score);
            continue;
        }
        if visited.contains(&(i, j, dir.clone())) {
            continue;
        }

        vec![
            (score + 1001, dir.turn_cw()),
            (score + 1001, dir.turn_ccw()),
            (score + 1, dir.clone()),
        ]
        .into_iter()
        .for_each(|(score, dir)| {
            let (i, j) = dir.step(i, j);
            if maze[i][j] != Maze::Wall {
                shortest
                    .entry((i, j, dir.clone()))
                    .and_modify(|cur_score| {
                        if score < *cur_score {
                            heap.push(Reverse((score, (i, j, dir.clone()))));
                            *cur_score = score;
                        }
                    })
                    .or_insert_with(|| {
                        heap.push(Reverse((score, (i, j, dir))));
                        score
                    });
            }
        });
        visited.insert((i, j, dir));
    }

    Ok(end.unwrap().to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    let Input { maze, start } = parse_input(input)?;

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, start.clone())));

    let mut shortest = HashMap::new();
    shortest.insert(start, (0, HashSet::new()));

    let mut visited = HashSet::new();

    let mut end_steps = HashSet::new();
    while end_steps.is_empty() {
        let (score, (i, j, dir)) = heap.pop().unwrap().0;
        if maze[i][j] == Maze::End {
            end_steps.insert((i, j));
            add_to_path(&mut end_steps, i, j, dir, &shortest);
            while heap
                .peek()
                .map(|peek| peek.0 .0 == score && peek.0 .1 .0 == i && peek.0 .1 .1 == j)
                .unwrap_or(false)
            {
                let (_, (i, j, dir)) = heap.pop().unwrap().0;
                add_to_path(&mut end_steps, i, j, dir, &shortest);
            }
            continue;
        }
        if visited.contains(&(i, j, dir.clone())) {
            continue;
        }

        vec![
            (score + 1001, dir.turn_cw()),
            (score + 1001, dir.turn_ccw()),
            (score + 1, dir.clone()),
        ]
        .into_iter()
        .for_each(|(score, next_dir)| {
            let (next_i, next_j) = next_dir.step(i, j);
            if maze[next_i][next_j] != Maze::Wall {
                shortest
                    .entry((next_i, next_j, next_dir.clone()))
                    .and_modify(|(cur_score, last)| match score.cmp(cur_score) {
                        std::cmp::Ordering::Less => {
                            heap.push(Reverse((score, (next_i, next_j, next_dir.clone()))));
                            *cur_score = score;
                            last.insert((i, j, dir.clone()));
                        }
                        std::cmp::Ordering::Equal => {
                            last.insert((i, j, dir.clone()));
                        }
                        std::cmp::Ordering::Greater => {}
                    })
                    .or_insert_with(|| {
                        heap.push(Reverse((score, (next_i, next_j, next_dir.clone()))));
                        let mut last = HashSet::new();
                        last.insert((i, j, dir.clone()));
                        (score, last)
                    });
            }
        });
        visited.insert((i, j, dir));
    }

    Ok(end_steps.len().to_string())
}

type ShortestMap = HashMap<(usize, usize, Dir), (i32, HashSet<(usize, usize, Dir)>)>;
fn add_to_path(
    end_path: &mut HashSet<(usize, usize)>,
    i: usize,
    j: usize,
    dir: Dir,
    shortest: &ShortestMap,
) {
    shortest
        .get(&(i, j, dir))
        .unwrap()
        .1
        .iter()
        .for_each(|(i, j, dir)| {
            end_path.insert((*i, *j));
            add_to_path(end_path, *i, *j, dir.clone(), shortest);
        });
}
