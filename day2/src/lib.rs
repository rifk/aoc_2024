use anyhow::Result;

pub fn solve_one(input: &str) -> Result<String> {
    let reports = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|v| Ok(v.parse::<i32>()?))
                .collect::<Result<Vec<i32>>>()
        })
        .collect::<Result<Vec<Vec<i32>>>>()?;
    Ok(reports
        .into_iter()
        .filter(|r| {
            r.windows(2)
                .map(|w| w[0] - w[1])
                .try_fold(0i32, |prev, cur| {
                    if (prev == 0 || prev.signum() == cur.signum()) && (1..=3).contains(&cur.abs())
                    {
                        Some(cur)
                    } else {
                        None
                    }
                })
                .is_some()
        })
        .count()
        .to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    let reports = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|v| Ok(v.parse::<i32>()?))
                .collect::<Result<Vec<i32>>>()
        })
        .collect::<Result<Vec<Vec<i32>>>>()?;
    Ok(reports
        .into_iter()
        .filter(|r| is_safe(r.iter(), false, None, None))
        .count()
        .to_string())
}

fn is_safe<'a>(
    mut iter: impl Iterator<Item = &'a i32> + Clone,
    skipped: bool,
    last: Option<i32>,
    last_diff: Option<i32>,
) -> bool {
    let cur = if let Some(&cur) = iter.next() {
        cur
    } else {
        return true;
    };

    if !skipped && is_safe(iter.clone(), true, last, last_diff) {
        return true;
    }

    let diff = if let Some(last) = last {
        last - cur
    } else {
        return is_safe(iter, skipped, Some(cur), last_diff);
    };

    if last_diff
        .map(|ld| ld.signum() == diff.signum())
        .unwrap_or(true)
        && (1..=3).contains(&diff.abs())
    {
        is_safe(iter, skipped, Some(cur), Some(diff))
    } else {
        false
    }
}
