use anyhow::Result;

pub fn solve_one(input: &str) -> Result<String> {
    let chars = &input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    Ok((0..chars.len())
        .flat_map(|i| (0..chars[i].len()).map(move |j| (i, j)))
        .map(|(i, j)| count_xmas(chars, i, j))
        .sum::<usize>()
        .to_string())
}
fn count_xmas(chars: &Vec<Vec<char>>, i: usize, j: usize) -> usize {
    enum Dir {
        N,
        S,
        E,
        W,
        NE,
        NW,
        SE,
        SW,
    }
    fn is_xmas(chars: &Vec<Vec<char>>, i: usize, j: usize, dir: Dir, next: char) -> bool {
        let Some((i, j)) = (|| {
            Some(match dir {
                Dir::N => (i.checked_sub(1)?, j),
                Dir::S => (i.checked_add(1)?, j),
                Dir::E => (i, j.checked_add(1)?),
                Dir::W => (i, j.checked_sub(1)?),
                Dir::NE => (i.checked_sub(1)?, j.checked_add(1)?),
                Dir::NW => (i.checked_sub(1)?, j.checked_sub(1)?),
                Dir::SE => (i.checked_add(1)?, j.checked_add(1)?),
                Dir::SW => (i.checked_add(1)?, j.checked_sub(1)?),
            })
        })() else {
            return false;
        };
        if (|| Some(*chars.get(i)?.get(j)? != next))().unwrap_or(true) {
            return false;
        }
        match next {
            'M' => is_xmas(chars, i, j, dir, 'A'),
            'A' => is_xmas(chars, i, j, dir, 'S'),
            _ => true,
        }
    }
    if chars[i][j] == 'X' {
        is_xmas(chars, i, j, Dir::N, 'M') as usize
            + is_xmas(chars, i, j, Dir::S, 'M') as usize
            + is_xmas(chars, i, j, Dir::E, 'M') as usize
            + is_xmas(chars, i, j, Dir::W, 'M') as usize
            + is_xmas(chars, i, j, Dir::NE, 'M') as usize
            + is_xmas(chars, i, j, Dir::NW, 'M') as usize
            + is_xmas(chars, i, j, Dir::SE, 'M') as usize
            + is_xmas(chars, i, j, Dir::SW, 'M') as usize
    } else {
        0
    }
}

pub fn solve_two(input: &str) -> Result<String> {
    let chars = &input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    Ok((0..chars.len())
        .flat_map(|i| (0..chars[i].len()).map(move |j| (i, j)))
        .filter(|&(i, j)| is_x_mas(chars, i, j))
        .count()
        .to_string())
}
fn is_x_mas(chars: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if chars[i][j] != 'A' {
        return false;
    }
    let Some(corners) = (|| {
        Some((
            chars.get(i.checked_sub(1)?)?.get(j.checked_sub(1)?)?,
            chars.get(i.checked_sub(1)?)?.get(j.checked_add(1)?)?,
            chars.get(i.checked_add(1)?)?.get(j.checked_sub(1)?)?,
            chars.get(i.checked_add(1)?)?.get(j.checked_add(1)?)?,
        ))
    })() else {
        return false;
    };
    match corners {
        ('M', 'M', 'S', 'S')
        | ('M', 'S', 'M', 'S')
        | ('S', 'S', 'M', 'M')
        | ('S', 'M', 'S', 'M') => true,
        _ => false,
    }
}
