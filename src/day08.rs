use crate::SolveInfo;

pub(crate) fn run(input: &str) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

fn part01(input: &str) -> usize {
    let mut grid: Vec<Vec<(u32, bool)>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| (ch.to_digit(10).unwrap(), false))
                .collect::<Vec<(u32, bool)>>()
        })
        .collect();
    // build a point map
    // let mut trees = HashMap::new();
    // let mut width = 0;
    // let mut height = 0;
    // for (row, line) in input.lines().enumerate() {
    //     for (col, tree) in line.chars().enumerate() {
    //         trees.insert((row, col), tree.to_digit(10).unwrap());
    //         width = col;
    //     }
    //     height = row;
    // }

    // top-left to bottom-right
    let deltas: [(i32, i32); 2] = [(-1, 0), (0, -1)];
    for (dr, dc) in deltas.into_iter() {
        let mut tree_heights = grid.clone();
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                let (h, is_visible) = grid[row][col];
                let is_visible = is_visible
                    || row == 0
                    || col == 0
                    || tree_heights[add(row, dr)][col].0 < h
                    || tree_heights[row][add(col, dc)].0 < h;
                grid[row][col].1 = is_visible;

                let maxh = [
                    Some(h),
                    if row == 0 {
                        None
                    } else {
                        Some(tree_heights[add(row, dr)][col].0)
                    },
                    if col == 0 {
                        None
                    } else {
                        Some(tree_heights[row][add(col, dc)].0)
                    },
                ]
                .into_iter()
                .flatten()
                .max()
                .unwrap();
                tree_heights[row][col].0 = maxh;
            }
        }
    }

    // bottom-right to top-left
    let deltas: [(i32, i32); 2] = [(1, 0), (0, 1)];
    for (dr, dc) in deltas.into_iter() {
        let mut tree_heights = grid.clone();
        for row in (0..grid.len()).rev() {
            for col in (0..grid[0].len()).rev() {
                let (h, is_visible) = grid[row][col];
                let is_visible = is_visible
                    || row == grid.len() - 1
                    || col == grid[0].len() - 1
                    || tree_heights[add(row, dr)][col].0 < h
                    || tree_heights[row][add(col, dc)].0 < h;
                grid[row][col].1 = is_visible;

                let maxh = [
                    Some(h),
                    if row == grid.len() - 1 {
                        None
                    } else {
                        Some(tree_heights[add(row, dr)][col].0)
                    },
                    if col == grid[0].len() - 1 {
                        None
                    } else {
                        Some(tree_heights[row][add(col, dc)].0)
                    },
                ]
                .into_iter()
                .flatten()
                .max()
                .unwrap();
                tree_heights[row][col].0 = maxh;
            }
        }
    }

    grid.into_iter()
        .flat_map(|row| row.into_iter().map(|(_, v)| v).collect::<Vec<bool>>())
        .filter(|v| *v)
        .count()
}

fn part02(input: &str) -> u32 {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    // brute force solution, i'm sure there is probably a memo solution
    let mut max = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let mut left = 0;
            for i in (0..col).rev() {
                left += 1;
                if grid[row][col] <= grid[row][i] {
                    break;
                }
            }
            let mut right = 0;
            for i in col + 1..grid[0].len() {
                right += 1;
                if grid[row][col] <= grid[row][i] {
                    break;
                }
            }
            let mut top = 0;
            for i in (0..row).rev() {
                top += 1;
                if grid[row][col] <= grid[i][col] {
                    break;
                }
            }
            let mut bot = 0;
            for i in row + 1..grid.len() {
                bot += 1;
                if grid[row][col] <= grid[i][col] {
                    break;
                }
            }

            max = max.max(left * right * top * bot);
        }
    }
    max
}

fn add(u: usize, i: i32) -> usize {
    if i.is_negative() {
        u - i.unsigned_abs() as usize
    } else {
        u + i as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = include_str!("../inputs/8.sample.txt");
    const INPUT: &'static str = include_str!("../inputs/8.input.txt");

    #[test]
    fn test_part_one_sample() {
        let ans = part01(SAMPLE);
        assert_eq!(21, ans);
    }

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT);
        assert_eq!(1785, ans);
    }

    #[test]
    fn test_part_two_sample() {
        let ans = part02(SAMPLE);
        assert_eq!(8, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT);
        assert_eq!(345168, ans);
    }
}