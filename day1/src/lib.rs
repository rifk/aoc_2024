use anyhow::Result;

pub fn solve_one(input: &str) -> Result<String> {
    let (mut l, mut r): (Vec<usize>, Vec<usize>) = input
        .split_whitespace()
        .map(|v| Ok(v.parse::<usize>()?))
        .collect::<Result<Vec<usize>>>()?
        .chunks(2)
        .map(|p| (p[0], p[1]))
        .unzip();
    l.sort();
    r.sort();
    Ok(l.iter()
        .zip(r.iter())
        .map(|(&l, &r)| l.abs_diff(r))
        .sum::<usize>()
        .to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    let (l, r): (Vec<usize>, Vec<usize>) = input
        .split_whitespace()
        .map(|v| Ok(v.parse::<usize>()?))
        .collect::<Result<Vec<usize>>>()?
        .chunks(2)
        .map(|p| (p[0], p[1]))
        .unzip();
    Ok(l.into_iter()
        .map(|l| l * r.iter().filter(|&r| &l == r).count())
        .sum::<usize>()
        .to_string())
}
