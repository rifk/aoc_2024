use anyhow::{anyhow, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Block {
    File(usize),
    Free,
}

fn parse_input(input: &str) -> Result<Vec<Block>> {
    Ok(input
        .trim()
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .enumerate()
        .map(|(id, file_free)| {
            let mut blocks = vec![];

            let file_len = file_free[0]
                .to_digit(10)
                .ok_or_else(|| anyhow!(format!("expecting digit - found {file_free:?}")))?;
            (0..file_len).for_each(|_| blocks.push(Block::File(id)));

            let free_len = file_free.get(1).map(|c| {
                c.to_digit(10)
                    .ok_or_else(|| anyhow!(format!("expecting digit - found {file_free:?}")))
            });
            if let Some(free_len) = free_len {
                (0..free_len?).for_each(|_| blocks.push(Block::Free));
            }

            Ok(blocks)
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect())
}

pub fn solve_one(input: &str) -> Result<String> {
    let mut blocks = parse_input(input)?;
    let mut start = 0;
    let mut end = blocks.len() - 1;
    while start < end {
        match blocks[start] {
            Block::File(_) => start += 1,
            Block::Free => match blocks[end] {
                Block::Free => end -= 1,
                Block::File(_) => blocks.swap(start, end),
            },
        }
    }
    Ok(blocks
        .into_iter()
        .enumerate()
        .map_while(|(pos, block)| match block {
            Block::File(id) => Some(pos * id),
            Block::Free => None,
        })
        .sum::<usize>()
        .to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    let mut blocks = parse_input(input)?;
    let mut start = 0;
    let mut end = blocks.len() - 1;
    while start < end {
        if matches!(blocks[start], Block::File(_)) {
            start += 1;
            continue;
        }

        let id = match blocks[end] {
            Block::Free => {
                end -= 1;
                continue;
            }
            Block::File(id) => {
                end -= 1;
                id
            }
        };
        let mut file_len = 1;
        while blocks[end] == Block::File(id) {
            end -= 1;
            file_len += 1;
        }

        let mut free = (start + 1, 1);
        while free.1 < file_len && free.0 <= end {
            match blocks[free.0] {
                Block::File(_) => free.1 = 0,
                Block::Free => free.1 += 1,
            }
            free.0 += 1;
        }

        if free.1 == file_len {
            let mut file = end;
            (0..file_len).for_each(|_| {
                free.0 -= 1;
                file += 1;
                blocks.swap(free.0, file);
            })
        }
    }
    Ok(blocks
        .into_iter()
        .enumerate()
        .filter_map(|(pos, block)| match block {
            Block::File(id) => Some(pos * id),
            Block::Free => None,
        })
        .sum::<usize>()
        .to_string())
}
