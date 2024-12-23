use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Result};

fn parse_input(input: &str) -> Result<HashMap<String, HashSet<String>>> {
    let conns = input
        .lines()
        .map(|line| line.split_once('-').ok_or_else(|| anyhow!("missing -")))
        .collect::<Result<Vec<_>>>()?;
    let mut comps = conns
        .iter()
        .flat_map(|(l, r)| vec![l, r])
        .collect::<HashSet<_>>()
        .iter()
        .map(|n| (n.to_string(), HashSet::new()))
        .collect::<HashMap<_, _>>();
    conns.into_iter().for_each(|(l, r)| {
        comps.get_mut(l).unwrap().insert(r.to_string());
        comps.get_mut(r).unwrap().insert(l.to_string());
    });
    Ok(comps)
}

pub fn solve_one(input: &str) -> Result<String> {
    let comps = &parse_input(input)?;
    let threes_with_t: HashSet<Vec<String>> = comps
        .iter()
        .flat_map(|(name1, conns1)| {
            conns1.iter().flat_map(move |name2| {
                comps
                    .get(name2)
                    .unwrap()
                    .intersection(conns1)
                    .filter_map(move |name3| {
                        if name1.starts_with('t')
                            || name2.starts_with('t')
                            || name3.starts_with('t')
                        {
                            let mut v = vec![name1.clone(), name2.clone(), name3.clone()];
                            v.sort();
                            Some(v)
                        } else {
                            None
                        }
                    })
                    .collect::<HashSet<_>>()
            })
        })
        .collect();
    Ok(threes_with_t.len().to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    let comps = &parse_input(input)?;
    let mut largest_conn =
        largest_connected(comps, HashSet::new(), comps.keys().cloned().collect())
            .into_iter()
            .collect::<Vec<_>>();
    largest_conn.sort();
    Ok(largest_conn.join(","))
}

fn largest_connected(
    all_comps: &HashMap<String, HashSet<String>>,
    cur_comps: HashSet<String>,
    mut shared_conns: HashSet<String>,
) -> HashSet<String> {
    if shared_conns.is_empty() {
        return cur_comps;
    }

    let next = shared_conns.iter().next().unwrap().clone();
    shared_conns.remove(&next);

    let next_conns = all_comps.get(&next).unwrap();

    let include_next = largest_connected(
        all_comps,
        {
            let mut cur = cur_comps.clone();
            cur.insert(next);
            cur
        },
        shared_conns.intersection(next_conns).cloned().collect(),
    );
    let exclude_next = largest_connected(all_comps, cur_comps, shared_conns);

    if include_next.len() > exclude_next.len() {
        include_next
    } else {
        exclude_next
    }
}
