use std::collections::HashMap;

use anyhow::Result;

pub fn solve_one(input: &str) -> Result<String> {
    let mut stones = input
        .trim_matches('\n')
        .split_whitespace()
        .map(|stone| Ok(stone.parse::<u64>()?))
        .collect::<Result<Vec<_>>>()?;
    for _ in 0..25 {
        stones = stones.into_iter().flat_map(blink).collect();
    }
    Ok(stones.len().to_string())
}

fn blink(stone: u64) -> Vec<u64> {
    if stone == 0 {
        vec![1]
    } else {
        let mut r = stone % 10;
        let mut l = stone / 10;
        let mut tens = 10;
        while l / tens != 0 {
            r += tens * (l % 10);
            tens *= 10;
            l /= 10;
        }

        if l * 10 / tens != 0 {
            vec![l, r]
        } else {
            vec![stone * 2024]
        }
    }
}

pub fn solve_two(input: &str) -> Result<String> {
    let stones = input
        .trim_matches('\n')
        .split_whitespace()
        .map(|stone| Ok(stone.parse::<u64>()?))
        .collect::<Result<Vec<_>>>()?;
    let mut mem = HashMap::new();
    Ok(stones
        .into_iter()
        .map(|stone| num_stones(&mut mem, stone, 75))
        .sum::<usize>()
        .to_string())
}

fn num_stones(mem: &mut HashMap<(u64, u32), usize>, stone: u64, count: u32) -> usize {
    if count == 0 {
        return 1;
    }
    if let Some(&num) = mem.get(&(stone, count)) {
        return num;
    }
    let new_stones = blink(stone);

    let num = num_stones(mem, new_stones[0], count - 1)
        + new_stones
            .get(1)
            .map(|&s| num_stones(mem, s, count - 1))
            .unwrap_or(0);
    mem.insert((stone, count), num);
    num
}
