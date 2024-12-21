use std::collections::HashMap;

use anyhow::{bail, Result};

static NUMERIC_KEYPAD: [[Option<Numeric>; 3]; 4] = [
    [Some(Numeric::D7), Some(Numeric::D8), Some(Numeric::D9)],
    [Some(Numeric::D4), Some(Numeric::D5), Some(Numeric::D6)],
    [Some(Numeric::D1), Some(Numeric::D2), Some(Numeric::D3)],
    [None, Some(Numeric::D0), Some(Numeric::A)],
];
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Numeric {
    D0,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
    A,
}
impl Numeric {
    fn new(c: char) -> Result<Self> {
        Ok(match c {
            '0' => Numeric::D0,
            '1' => Numeric::D1,
            '2' => Numeric::D2,
            '3' => Numeric::D3,
            '4' => Numeric::D4,
            '5' => Numeric::D5,
            '6' => Numeric::D6,
            '7' => Numeric::D7,
            '8' => Numeric::D8,
            '9' => Numeric::D9,
            'A' => Numeric::A,
            _ => bail!("unexpected char {c}"),
        })
    }
    fn seqs_to_press(&self, to: &Numeric) -> Vec<Vec<Dir>> {
        let mut seqs = vec![(self.clone(), vec![])];
        while seqs.iter().all(|(b, _)| b != to) {
            seqs = seqs
                .iter()
                .flat_map(|(b, s)| {
                    b.dirs()
                        .into_iter()
                        .map(|(d, n)| {
                            (n, {
                                let mut s = s.clone();
                                s.push(d);
                                s
                            })
                        })
                        .collect::<Vec<_>>()
                })
                .collect();
        }
        seqs.into_iter()
            .filter_map(|(b, mut s)| {
                if b == *to {
                    s.push(Dir::A);
                    Some(s)
                } else {
                    None
                }
            })
            .collect()
    }
    fn dirs(&self) -> Vec<(Dir, Numeric)> {
        let (i, j) = NUMERIC_KEYPAD
            .iter()
            .enumerate()
            .flat_map(|(i, r)| r.iter().enumerate().map(move |(j, n)| (i, j, n)))
            .find_map(|(i, j, n)| {
                if n.as_ref() == Some(self) {
                    Some((i, j))
                } else {
                    None
                }
            })
            .unwrap();
        vec![
            i.checked_add(1)
                .filter(|i| *i < NUMERIC_KEYPAD.len())
                .map(|i| (i, j, Dir::D)),
            i.checked_sub(1).map(|i| (i, j, Dir::U)),
            j.checked_add(1)
                .filter(|j| *j < NUMERIC_KEYPAD[i].len())
                .map(|j| (i, j, Dir::R)),
            j.checked_sub(1).map(|j| (i, j, Dir::L)),
        ]
        .into_iter()
        .flatten()
        .filter_map(|(i, j, d)| NUMERIC_KEYPAD[i][j].as_ref().map(|n| (d, n.clone())))
        .collect()
    }
}

static DIR_KEYPAD: [[Option<Dir>; 3]; 2] = [
    [None, Some(Dir::U), Some(Dir::A)],
    [Some(Dir::L), Some(Dir::D), Some(Dir::R)],
];
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Dir {
    U,
    D,
    L,
    R,
    A,
}
impl Dir {
    fn seqs_to_press(&self, to: &Dir) -> Vec<Vec<Dir>> {
        let mut seqs = vec![(self.clone(), vec![])];
        while seqs.iter().all(|(b, _)| b != to) {
            seqs = seqs
                .iter()
                .flat_map(|(b, s)| {
                    b.dirs()
                        .into_iter()
                        .map(|(d, n)| {
                            (n, {
                                let mut s = s.clone();
                                s.push(d);
                                s
                            })
                        })
                        .collect::<Vec<_>>()
                })
                .collect();
        }
        seqs.into_iter()
            .filter_map(|(b, mut s)| {
                if b == *to {
                    s.push(Dir::A);
                    Some(s)
                } else {
                    None
                }
            })
            .collect()
    }

    fn dirs(&self) -> Vec<(Dir, Dir)> {
        let (i, j) = DIR_KEYPAD
            .iter()
            .enumerate()
            .flat_map(|(i, r)| r.iter().enumerate().map(move |(j, n)| (i, j, n)))
            .find_map(|(i, j, n)| {
                if n.as_ref() == Some(self) {
                    Some((i, j))
                } else {
                    None
                }
            })
            .unwrap();
        vec![
            i.checked_add(1)
                .filter(|i| *i < DIR_KEYPAD.len())
                .map(|i| (i, j, Dir::D)),
            i.checked_sub(1).map(|i| (i, j, Dir::U)),
            j.checked_add(1)
                .filter(|j| *j < DIR_KEYPAD[i].len())
                .map(|j| (i, j, Dir::R)),
            j.checked_sub(1).map(|j| (i, j, Dir::L)),
        ]
        .into_iter()
        .flatten()
        .filter_map(|(i, j, d)| DIR_KEYPAD[i][j].as_ref().map(|n| (d, n.clone())))
        .collect()
    }
}

fn parse_input(input: &str) -> Result<Vec<(Vec<Numeric>, u64)>> {
    input
        .lines()
        .map(|line| {
            let mut n = 0;
            line.chars()
                .map(|c| {
                    if let Some(d) = c.to_digit(10) {
                        n *= 10;
                        n += d as u64;
                    }
                    Numeric::new(c)
                })
                .collect::<Result<Vec<_>>>()
                .map(|code| (code, n))
        })
        .collect()
}

fn complexity(
    mem: &mut HashMap<(Dir, Dir, u8), u64>,
    code: &[Numeric],
    code_num: u64,
    dir_keypads: u8,
) -> u64 {
    let mut last_n = &Numeric::A;
    let mut len = 0;
    for n in code {
        // first robot, numeric keypad
        let dir_seqs = last_n.seqs_to_press(n);

        // me and remaining robots, dir keypad
        len += min_length_seq(mem, &dir_seqs, dir_keypads);

        last_n = n;
    }
    len * code_num
}

fn min_length_seq(mem: &mut HashMap<(Dir, Dir, u8), u64>, seqs: &[Vec<Dir>], keypads: u8) -> u64 {
    seqs.iter()
        .map(|s| {
            let mut len = 0;
            let mut last_d = &Dir::A;
            for d in s {
                len += min_length_press(mem, last_d, d, keypads);
                last_d = d;
            }
            len
        })
        .min()
        .unwrap()
}

fn min_length_press(
    mem: &mut HashMap<(Dir, Dir, u8), u64>,
    from: &Dir,
    to: &Dir,
    keypads_left: u8,
) -> u64 {
    if keypads_left == 0 {
        return 1;
    }
    let k = (from.clone(), to.clone(), keypads_left);
    if let Some(min) = mem.get(&k) {
        return *min;
    }
    let seqs = from.seqs_to_press(to);
    let min = min_length_seq(mem, &seqs, keypads_left - 1);
    mem.insert(k, min);
    min
}

pub fn solve_one(input: &str) -> Result<String> {
    let codes = parse_input(input)?;
    let mut mem = HashMap::new();
    Ok(codes
        .into_iter()
        .map(|(code, n)| complexity(&mut mem, &code, n, 2))
        .sum::<u64>()
        .to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    let codes = parse_input(input)?;
    let mut mem = HashMap::new();
    Ok(codes
        .into_iter()
        .map(|(code, n)| complexity(&mut mem, &code, n, 25))
        .sum::<u64>()
        .to_string())
}
