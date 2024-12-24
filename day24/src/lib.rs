use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, bail, Result};

#[derive(Clone, Debug, PartialEq, Eq)]
enum GateType {
    And,
    Xor,
    Or,
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct Gate {
    gate_type: GateType,
    in1: String,
    in2: String,
    out: String,
}
impl Gate {
    fn resolve(&self, wires: &HashMap<String, bool>) -> Option<(String, bool)> {
        wires
            .get(&self.in1)
            .zip(wires.get(&self.in2))
            .map(|(in1, in2)| {
                (
                    self.out.clone(),
                    match self.gate_type {
                        GateType::And => in1 & in2,
                        GateType::Xor => in1 ^ in2,
                        GateType::Or => in1 | in2,
                    },
                )
            })
    }
    fn swap_outs(&mut self, other: &mut Self) {
        std::mem::swap(&mut self.out, &mut other.out);
    }
    fn match_in(&self, in_s: &str) -> Option<&str> {
        if self.in1 == in_s {
            Some(&self.in2)
        } else if self.in2 == in_s {
            Some(&self.in1)
        } else {
            None
        }
    }
}

fn parse_input(input: &str) -> Result<(HashMap<String, bool>, Vec<Gate>)> {
    let (wires, gates) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("missing empty line"))?;
    Ok((
        wires
            .lines()
            .map(|line| {
                line.split_once(": ")
                    .ok_or_else(|| anyhow!("missing ': '"))
                    .and_then(|(k, v)| {
                        Ok((
                            k.to_string(),
                            match v {
                                "0" => false,
                                "1" => true,
                                _ => bail!("unexpected {v}"),
                            },
                        ))
                    })
            })
            .collect::<Result<HashMap<_, _>>>()?,
        gates
            .lines()
            .map(|line| {
                let and = line.split_once(" AND ");
                let xor = line.split_once(" XOR ");
                let or = line.split_once(" OR ");
                let (gate_type, in1, rest) = match (and, xor, or) {
                    (Some((l, rest)), None, None) => (GateType::And, l, rest),
                    (None, Some((l, rest)), None) => (GateType::Xor, l, rest),
                    (None, None, Some((l, rest))) => (GateType::Or, l, rest),
                    _ => bail!("unexpected gate: {line}"),
                };
                rest.split_once(" -> ")
                    .ok_or_else(|| anyhow!("missing ->"))
                    .map(|(in2, out)| Gate {
                        gate_type,
                        in1: in1.to_string(),
                        in2: in2.to_string(),
                        out: out.to_string(),
                    })
            })
            .collect::<Result<Vec<_>>>()?,
    ))
}

fn get_z(mut wires: HashMap<String, bool>, gates: &[Gate]) -> Option<u64> {
    let mut z: u64 = 0;
    let mut gates = gates.iter().collect::<Vec<&Gate>>();
    let mut wires_count = wires.len();
    while !gates.is_empty() {
        gates.retain(|gate| {
            if let Some((wire, res)) = gate.resolve(&wires) {
                if res && wire.starts_with('z') {
                    let z_i = wire[1..].parse::<usize>().unwrap();
                    z |= 1 << z_i;
                }
                wires.insert(wire, res);
                false
            } else {
                true
            }
        });
        let next_count = wires.len();
        if wires_count == next_count {
            return None;
        }
        wires_count = next_count;
    }
    Some(z)
}

pub fn solve_one(input: &str) -> Result<String> {
    let (wires, gates) = parse_input(input)?;
    Ok(get_z(wires, &gates).unwrap().to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    let (wires, mut gates) = parse_input(input)?;
    let gates = &mut gates;
    let x_y_len = wires
        .keys()
        .map(|wire| {
            wire.trim_start_matches("x")
                .trim_start_matches("y")
                .parse::<usize>()
                .unwrap()
                + 1
        })
        .max()
        .unwrap();

    let mut swaps = HashSet::<usize>::new();
    let mut used_gates = HashSet::new();
    let x_y_0_xor_i = find_gate_from_two_in(gates, "x00", "y00", GateType::Xor).unwrap();
    assert!(gates[x_y_0_xor_i].out == "z00");

    let carry_i = find_gate_from_two_in(gates, "x00", "y00", GateType::And).unwrap();
    used_gates.insert(x_y_0_xor_i);
    used_gates.insert(carry_i);

    build_adder(x_y_len, gates, &mut used_gates, &mut swaps, carry_i, 1);

    let mut swaps_out = swaps
        .into_iter()
        .map(|i| gates[i].out.clone())
        .collect::<Vec<_>>();
    swaps_out.sort();
    Ok(swaps_out.join(","))
}

fn find_gate_from_two_in(
    gates: &[Gate],
    in1: &str,
    in2: &str,
    gate_type: GateType,
) -> Option<usize> {
    gates
        .iter()
        .enumerate()
        .find(|(_, gate)| {
            gate.gate_type == gate_type
                && (gate.match_in(in1) == Some(in2) || gate.match_in(in2) == Some(in1))
        })
        .map(|(i, _)| i)
}

fn find_gate_from_one_in(gates: &[Gate], in1: &str, gate_type: GateType) -> Option<usize> {
    gates
        .iter()
        .enumerate()
        .find(|(_, gate)| gate.gate_type == gate_type && gate.match_in(in1).is_some())
        .map(|(i, _)| i)
}

fn find_gates_from_out(gates: &[Gate], out: &str) -> usize {
    gates
        .iter()
        .enumerate()
        .find(|(_, gate)| gate.out == out)
        .map(|(i, _)| i)
        .unwrap()
}

fn swap_gates(gates: &mut [Gate], i1: usize, i2: usize) {
    let (l_i, r_i) = (i1.min(i2), i1.max(i2));
    let (l, r) = gates.split_at_mut(r_i);
    l[l_i].swap_outs(&mut r[0]);
}

fn build_adder(
    x_y_len: usize,
    gates: &mut [Gate],
    used_gates: &mut HashSet<usize>,
    swaps: &mut HashSet<usize>,
    mut carry_i: usize,
    x_y_i: usize,
) {
    if x_y_i == x_y_len {
        return;
    }

    let x_y_and_i = find_gate_from_two_in(
        gates,
        &format!("x{x_y_i:02}"),
        &format!("y{x_y_i:02}"),
        GateType::And,
    )
    .unwrap();
    used_gates.insert(x_y_and_i);
    let x_y_xor_i = find_gate_from_two_in(
        gates,
        &format!("x{x_y_i:02}"),
        &format!("y{x_y_i:02}"),
        GateType::Xor,
    )
    .unwrap();
    used_gates.insert(x_y_xor_i);

    if let Some(z_out_i) = find_gate_from_two_in(
        gates,
        &gates[x_y_xor_i].out,
        &gates[carry_i].out,
        GateType::Xor,
    ) {
        used_gates.insert(z_out_i);
        if gates[z_out_i].out != format!("z{x_y_i:02}") {
            // z_out has incorrect out, swap it
            let swap_i = find_gates_from_out(gates, &format!("z{x_y_i:02}"));
            assert!(swaps.insert(swap_i));
            assert!(swaps.insert(z_out_i));
            swap_gates(gates, z_out_i, swap_i);
        }
    } else {
        // either carry or x_y_xor has incorrect out - need swap
        let z_out_i = find_gates_from_out(gates, &format!("z{x_y_i:02}"));
        let z_out = &gates[z_out_i];
        if let Some(other_in) = z_out.match_in(&gates[carry_i].out) {
            // carry correct, swap x_y_xor.out for other_in
            let other_in_i = find_gates_from_out(gates, other_in);
            assert!(swaps.insert(other_in_i));
            assert!(swaps.insert(x_y_xor_i));
            swap_gates(gates, other_in_i, x_y_xor_i);
        } else if let Some(other_in) = z_out.match_in(&gates[x_y_xor_i].out) {
            // x_y_xor correct, swap carry.out for other_in
            let other_in_i = find_gates_from_out(gates, other_in);
            assert!(swaps.insert(other_in_i));
            assert!(swaps.insert(carry_i));
            swap_gates(gates, other_in_i, carry_i);
        } else {
            panic!("gate with z_out doenst match either in")
        }
    }

    // carry and x_y_xor are correct, gate "and" them
    let carry_and_i = find_gate_from_two_in(
        gates,
        &gates[carry_i].out,
        &gates[x_y_xor_i].out,
        GateType::And,
    )
    .unwrap();
    used_gates.insert(carry_and_i);

    if let Some(next_carry_i) = find_gate_from_two_in(
        gates,
        &gates[x_y_and_i].out,
        &gates[carry_and_i].out,
        GateType::Or,
    ) {
        used_gates.insert(next_carry_i);
        carry_i = next_carry_i;
    } else {
        // either x_y_and_i out or carry_and_i out is wrong
        let next_carry_i = match (
            find_gate_from_one_in(gates, &gates[x_y_and_i].out, GateType::Or),
            find_gate_from_one_in(gates, &gates[carry_and_i].out, GateType::Or),
        ) {
            (None, Some(other_i)) => {
                // carry_add_i out found in Or, swap x_y_and_i out with other
                assert!(swaps.insert(other_i));
                assert!(swaps.insert(x_y_and_i));
                swap_gates(gates, other_i, x_y_and_i);
                other_i
            }
            (Some(other_i), None) => {
                // x_y_and_i out found in Or, carry_and_i out with other
                assert!(swaps.insert(other_i));
                assert!(swaps.insert(carry_and_i));
                swap_gates(gates, other_i, carry_and_i);
                other_i
            }
            _ => panic!("expecting single match"),
        };
        used_gates.insert(next_carry_i);
        carry_i = next_carry_i;
    }

    build_adder(x_y_len, gates, used_gates, swaps, carry_i, x_y_i + 1);
}
