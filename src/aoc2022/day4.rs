use anyhow::{anyhow, Result};

fn get_input() -> &'static str {
    include_str!("./inputs/day4.real")
}

fn range_to_hashset(r: &'static str) -> Result<std::collections::HashSet<u32>> {
    let (start, end) = r.split_once('-').ok_or(anyhow!("could not split range"))?;
    let start = start.parse::<u32>()?;
    let end = end.parse::<u32>()?;

    Ok((start..=end).collect())
}

pub fn solve_one() {
    let x: usize = get_input()
        .lines()
        .flat_map(|line| line.split_once(','))
        .flat_map(|(a, b)| -> Result<_> {
            let ha = range_to_hashset(a)?;
            let hb = range_to_hashset(b)?;
            Ok((ha, hb))
        })
        .map(|(ha, hb)| usize::from(ha.is_superset(&hb) || hb.is_superset(&ha)))
        .sum();
    println!("{x}")
}

pub fn solve_two() {
    let x: usize = get_input()
        .lines()
        .flat_map(|line| line.split_once(','))
        .flat_map(|(a, b)| -> Result<_> {
            let ha = range_to_hashset(a)?;
            let hb = range_to_hashset(b)?;
            Ok((ha, hb))
        })
        .map(|(ha, hb)| usize::from(!ha.is_disjoint(&hb)))
        .sum();
    println!("{x}")
}
