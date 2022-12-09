use std::collections::HashSet;

use crate::SolveInfo;

pub(crate) fn run(input: &str) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

fn part01(input: &str) -> usize {
    solve(input, 2)
}

fn part02(input: &str) -> usize {
    solve(input, 10)
}

fn solve(input: &str, n: usize) -> usize {
    let mut visited = HashSet::new();
    let mut knots = vec![(0, 0); n];
    visited.insert((0, 0));
    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(dir, n)| (dir, n.parse::<u32>().unwrap()))
        .for_each(|(dir, n)| {
            for _ in 0..n {
                match dir {
                    "R" => knots[0] = (knots[0].0, knots[0].1 + 1),
                    "L" => knots[0] = (knots[0].0, knots[0].1 - 1),
                    "U" => knots[0] = (knots[0].0 + 1, knots[0].1),
                    "D" => knots[0] = (knots[0].0 - 1, knots[0].1),
                    _ => panic!("unknown direction {}", dir),
                }

                for i in 1..knots.len() {
                    let dx = knots[i - 1].1 - knots[i].1;
                    let dy = knots[i - 1].0 - knots[i].0;
                    match (dx, dy) {
                        (0, 2) => knots[i].0 += 1,
                        (0, -2) => knots[i].0 -= 1,
                        (2, 0) => knots[i].1 += 1,
                        (-2, 0) => knots[i].1 -= 1,
                        (1, 2) | (2, 1) | (2, 2) => knots[i] = (knots[i].0 + 1, knots[i].1 + 1),
                        (-1, 2) | (-2, 1) | (-2, 2) => knots[i] = (knots[i].0 + 1, knots[i].1 - 1),
                        (1, -2) | (2, -1) | (2, -2) => knots[i] = (knots[i].0 - 1, knots[i].1 + 1),
                        (-1, -2) | (-2, -1) | (-2, -2) => {
                            knots[i] = (knots[i].0 - 1, knots[i].1 - 1)
                        }
                        _ => {}
                    }

                    if i == knots.len() - 1 {
                        visited.insert(knots[knots.len() - 1]);
                    }
                }
            }
        });

    visited.into_iter().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = include_str!("../inputs/9.sample.txt");
    const SAMPLE2: &'static str = include_str!("../inputs/9.sample2.txt");
    const INPUT: &'static str = include_str!("../inputs/9.input.txt");

    #[test]
    fn test_part_one_sample() {
        let ans = part01(SAMPLE);
        assert_eq!(13, ans);
    }

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT);
        assert_eq!(6384, ans);
    }

    #[test]
    fn test_part_two_sample() {
        let ans = part02(SAMPLE);
        assert_eq!(1, ans);
    }

    #[test]
    fn test_part_two_sample2() {
        let ans = part02(SAMPLE2);
        assert_eq!(36, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT);
        assert_eq!(2734, ans);
    }
}