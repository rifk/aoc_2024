use anyhow::{anyhow, Result};

struct Coord {
    x: i64,
    y: i64,
}

fn parse_input(input: &str, prize_add: i64) -> Result<Vec<(Coord, Coord, Coord)>> {
    input
        .split("\n\n")
        .map(|s| {
            let s = s.trim();
            let s = s.trim_start_matches("Button A: X+");
            let (a_x, s) = s
                .split_once(", Y+")
                .ok_or_else(|| anyhow!("couldnt split button A by Y+"))?;
            let a_x = a_x.parse::<i64>()?;
            let (a_y, s) = s
                .split_once("\nButton B: X+")
                .ok_or_else(|| anyhow!("couldnt split \\n button B"))?;
            let a_y = a_y.parse::<i64>()?;

            let (b_x, s) = s
                .split_once(", Y+")
                .ok_or_else(|| anyhow!("counldnt split button B by Y+"))?;
            let b_x = b_x.parse::<i64>()?;
            let (b_y, s) = s
                .split_once("\nPrize: X=")
                .ok_or_else(|| anyhow!("couldnt split \\n prize"))?;
            let b_y = b_y.parse::<i64>()?;

            let (p_x, p_y) = s
                .split_once(", Y=")
                .ok_or_else(|| anyhow!("couldnt split prize by Y="))?;
            let p_x = p_x.parse::<i64>()?;
            let p_y = p_y.parse::<i64>()?;

            Ok((
                Coord { x: a_x, y: a_y },
                Coord { x: b_x, y: b_y },
                Coord {
                    x: p_x + prize_add,
                    y: p_y + prize_add,
                },
            ))
        })
        .collect::<Result<Vec<_>>>()
}

pub fn solve_one(input: &str) -> Result<String> {
    Ok(parse_input(input, 0)?
        .into_iter()
        .filter_map(|(a, b, p)| {
            let a_max = (p.x / a.x).min(p.y / a.y);
            (0..=a_max)
                .filter_map(|a_n| {
                    let rem = Coord {
                        x: p.x - (a_n * a.x),
                        y: p.y - (a_n * a.y),
                    };
                    if rem.x % b.x == 0 && rem.y % b.y == 0 && rem.x / b.x == rem.y / b.y {
                        Some((3 * a_n) + (rem.x / b.x))
                    } else {
                        None
                    }
                })
                .min()
        })
        .sum::<i64>()
        .to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    Ok(parse_input(input, 10000000000000)?
        .into_iter()
        .filter_map(|(a, b, p)| {
            //bn = (ay px - ax py) / (bx ay - by ax)
            if ((a.y * p.x) - (a.x * p.y)) % ((b.x * a.y) - (b.y * a.x)) == 0 {
                Some(
                    (((a.y * p.x) - (a.x * p.y)) / ((b.x * a.y) - (b.y * a.x)))
                        + (3 * (((b.y * p.x) - (b.x * p.y)) / ((a.x * b.y) - (a.y * b.x)))),
                )
            } else {
                None
            }
        })
        .sum::<i64>()
        .to_string())
}
