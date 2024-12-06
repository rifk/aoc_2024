use anyhow::{anyhow, Result};
use std::{cell::Cell, collections::HashSet};

#[derive(Clone, Hash, Eq, PartialEq)]
enum Dir {
    N,
    S,
    E,
    W,
}
impl Dir {
    fn turn(&self) -> Self {
        match self {
            Dir::N => Dir::E,
            Dir::S => Dir::W,
            Dir::E => Dir::S,
            Dir::W => Dir::N,
        }
    }
}

struct ParsedInput {
    guard: (usize, usize),
    objects: HashSet<(usize, usize)>,
    max_i_j: (usize, usize),
}

pub fn solve_one(input: &str) -> Result<String> {
    let ParsedInput {
        guard,
        objects,
        max_i_j: (max_i, max_j),
    } = parse_input(input)?;

    Ok(get_steps(guard, max_i, max_j, &objects).len().to_string())
}

fn parse_input(input: &str) -> Result<ParsedInput> {
    let guard = &Cell::new(None);
    let objects = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(j, char)| match char {
                    '#' => Some((i, j)),
                    '^' => {
                        guard.replace(Some((i, j)));
                        None
                    }
                    _ => None,
                })
        })
        .collect::<HashSet<(usize, usize)>>();
    let guard = guard.get().ok_or_else(|| anyhow!("missing ^"))?;
    let max_i_j = (input.lines().count(), input.lines().next().unwrap().len());
    Ok(ParsedInput {
        guard,
        objects,
        max_i_j,
    })
}

fn get_steps(
    mut guard: (usize, usize),
    max_i: usize,
    max_j: usize,
    objects: &HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let mut dir = Dir::N;
    let mut steps = HashSet::new();
    steps.insert(guard);

    while let Some(next_guard) = match dir {
        Dir::N => guard.0.checked_sub(1).map(|i| (i, guard.1)),
        Dir::S => guard.0.checked_add(1).map(|i| (i, guard.1)),
        Dir::E => guard.1.checked_add(1).map(|j| (guard.0, j)),
        Dir::W => guard.1.checked_sub(1).map(|j| (guard.0, j)),
    } {
        if next_guard.0 >= max_i || next_guard.1 >= max_j {
            break;
        }
        if objects.contains(&next_guard) {
            dir = dir.turn();
            continue;
        }
        guard = next_guard;
        steps.insert(guard);
    }
    steps
}

pub fn solve_two(input: &str) -> Result<String> {
    let ParsedInput {
        guard,
        mut objects,
        max_i_j: (max_i, max_j),
    } = parse_input(input)?;

    let steps = get_steps(guard, max_i, max_j, &objects);

    Ok(steps
        .into_iter()
        .filter(|&step| {
            if guard == step {
                return false;
            }

            objects.insert(step);
            let l = is_loop(guard, max_i, max_j, &objects);
            objects.remove(&step);
            l
        })
        .count()
        .to_string())
}

fn is_loop(
    mut guard: (usize, usize),
    max_i: usize,
    max_j: usize,
    objects: &HashSet<(usize, usize)>,
) -> bool {
    let mut dir = Dir::N;
    let mut steps = HashSet::new();
    let mut steps_dir = HashSet::new();
    steps.insert(guard);
    steps_dir.insert((guard, dir.clone()));

    while let Some(next_guard) = match dir {
        Dir::N => guard.0.checked_sub(1).map(|i| (i, guard.1)),
        Dir::S => guard.0.checked_add(1).map(|i| (i, guard.1)),
        Dir::E => guard.1.checked_add(1).map(|j| (guard.0, j)),
        Dir::W => guard.1.checked_sub(1).map(|j| (guard.0, j)),
    } {
        if next_guard.0 >= max_i || next_guard.1 >= max_j {
            break;
        }
        if objects.contains(&next_guard) {
            dir = dir.turn();
            continue;
        }
        if steps_dir.contains(&(next_guard, dir.clone())) {
            return true;
        }
        guard = next_guard;
        steps.insert(guard);
        steps_dir.insert((guard, dir.clone()));
    }

    false
}
