use anyhow::{anyhow, bail, Result};

enum CompStep {
    Continue,
    Halt,
    Output(u64),
}

struct Comp {
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u64>,
    ptr: usize,
}
impl Comp {
    fn output(&mut self) -> Result<Vec<u64>> {
        let mut out = vec![];
        loop {
            match self.step()? {
                CompStep::Continue => {}
                CompStep::Halt => return Ok(out),
                CompStep::Output(output) => {
                    out.push(output);
                }
            }
        }
    }
    fn step(&mut self) -> Result<CompStep> {
        Ok(
            match (self.program.get(self.ptr), self.program.get(self.ptr + 1)) {
                (None, None) => CompStep::Halt,
                (Some(0), Some(operand)) => {
                    self.a /= 1 << self.combo_operand(*operand)?;
                    self.ptr += 2;
                    CompStep::Continue
                }
                (Some(1), Some(operand)) => {
                    self.b ^= operand;
                    self.ptr += 2;
                    CompStep::Continue
                }
                (Some(2), Some(operand)) => {
                    self.b = self.combo_operand(*operand)? % 8;
                    self.ptr += 2;
                    CompStep::Continue
                }
                (Some(3), Some(operand)) => {
                    if self.a == 0 {
                        self.ptr += 2;
                    } else {
                        self.ptr = *operand as usize;
                    }
                    CompStep::Continue
                }
                (Some(4), Some(_)) => {
                    self.b ^= self.c;
                    self.ptr += 2;
                    CompStep::Continue
                }
                (Some(5), Some(operand)) => {
                    self.ptr += 2;
                    CompStep::Output(self.combo_operand(*operand)? % 8)
                }
                (Some(6), Some(operand)) => {
                    self.b = self.a / (1 << self.combo_operand(*operand)?);
                    self.ptr += 2;
                    CompStep::Continue
                }
                (Some(7), Some(operand)) => {
                    self.c = self.a / (1 << self.combo_operand(*operand)?);
                    self.ptr += 2;
                    CompStep::Continue
                }
                op => bail!("unexpected opcode operand: {op:?}"),
            },
        )
    }
    fn combo_operand(&self, operand: u64) -> Result<u64> {
        Ok(match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => bail!("7 combo op"),
            _ => bail!("unexpected combo operand {operand}"),
        })
    }

    fn reset_with_a(&mut self, a: u64) {
        self.a = a;
        self.ptr = 0;
    }
}

fn parse_input(input: &str) -> Result<Comp> {
    let mut lines = input.lines();
    Ok(Comp {
        a: lines
            .next()
            .ok_or_else(|| anyhow!("missing A line"))?
            .trim_start_matches("Register A: ")
            .parse::<u64>()?,
        b: lines
            .next()
            .ok_or_else(|| anyhow!("missing B line"))?
            .trim_start_matches("Register B: ")
            .parse::<u64>()?,
        c: lines
            .next()
            .ok_or_else(|| anyhow!("missing C line"))?
            .trim_start_matches("Register C: ")
            .parse::<u64>()?,
        program: lines
            .nth(1)
            .ok_or_else(|| anyhow!("missing program line"))?
            .trim_start_matches("Program: ")
            .split(',')
            .map(|v| v.parse::<u64>().map_err(|e| anyhow!(e)))
            .collect::<Result<Vec<_>>>()?,
        ptr: 0,
    })
}

pub fn solve_one(input: &str) -> Result<String> {
    let mut comp = parse_input(input)?;
    Ok(comp
        .output()?
        .into_iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(","))
}

pub fn solve_two(input: &str) -> Result<String> {
    let mut comp = parse_input(input)?;
    let mut pos_a = (1..=8).collect::<Vec<_>>();
    let mut full_match = false;
    while !full_match && !pos_a.is_empty() {
        pos_a = pos_a
            .into_iter()
            .flat_map(|last_a| (0..8).map(move |rem| (last_a * 8) + rem))
            .filter_map(|a| {
                comp.reset_with_a(a);
                match comp.output() {
                    Ok(output) => match output.len().cmp(&comp.program.len()) {
                        std::cmp::Ordering::Less => {
                            if comp.program.ends_with(&output) {
                                Some(Ok(a))
                            } else {
                                None
                            }
                        }
                        std::cmp::Ordering::Equal => {
                            if output == comp.program {
                                full_match = true;
                                Some(Ok(a))
                            } else {
                                None
                            }
                        }
                        std::cmp::Ordering::Greater => None,
                    },
                    Err(e) => Some(Err(anyhow!(e))),
                }
            })
            .collect::<Result<_>>()?
    }
    Ok(pos_a
        .into_iter()
        .min()
        .ok_or_else(|| anyhow!("no possible a"))?
        .to_string())
}
