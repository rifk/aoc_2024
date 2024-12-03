use anyhow::Result;

pub fn solve_one(input: &str) -> Result<String> {
    Ok(input
        .split("mul(")
        .skip(1)
        .filter_map(|s| {
            let (l, s) = s.split_once(',')?;
            let l = l.parse::<usize>().ok()?;
            let (r, _) = s.split_once(')')?;
            let r = r.parse::<usize>().ok()?;
            Some(l * r)
        })
        .sum::<usize>()
        .to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    let mut enabled = true;
    let mut first = true;
    Ok(input
        .split("mul(")
        .filter_map(|s| {
            let mul = Some(s).filter(|_| enabled && !first).and_then(|s| {
                let (l, s) = s.split_once(',')?;
                let l = l.parse::<usize>().ok()?;
                let (r, _) = s.split_once(')')?;
                let r = r.parse::<usize>().ok()?;
                Some(l * r)
            });

            if first {
                first = false;
            }

            let do_i = s.rfind("do()");
            let dont_i = s.rfind("don't()");
            enabled = match (do_i, dont_i) {
                (None, None) => enabled,
                (None, Some(_)) => false,
                (Some(_), None) => true,
                (Some(do_i), Some(dont_i)) => do_i > dont_i,
            };

            mul
        })
        .sum::<usize>()
        .to_string())
}
