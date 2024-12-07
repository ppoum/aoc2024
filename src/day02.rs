fn _part1_take1(lines: Vec<String>) -> usize {
    let reports = lines.into_iter().map(|s| {
        s.split_whitespace()
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    });

    let mut total = 0;
    'report_loop: for report in reports {
        let mut increasing = false;
        let mut first_n = 0;
        let mut last_n = 0;
        for (i, n) in report.into_iter().enumerate() {
            if i == 0 {
                first_n = n;
                last_n = n;
                continue;
            } else if i == 1 {
                if n == first_n {
                    // Invalid report
                    continue 'report_loop;
                }
                increasing = n > first_n;
            }

            let diff = n as isize - last_n as isize;
            last_n = n;
            if increasing && !(1..=3).contains(&diff) || !increasing && !(-3..=-1).contains(&diff) {
                // Invalid report (delta too large or wrong direction)
                continue 'report_loop;
            }
        }
        total += 1
    }
    total
}

pub fn part1(lines: Vec<String>) -> usize {
    let reports = lines.into_iter().map(|s| {
        s.split_whitespace()
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    });

    // Map report [1,3,5,9] to delta array [2,2,4]
    let deltas = reports.map(|report| {
        report
            .windows(2)
            .map(|win| win[1] as isize - win[0] as isize)
            .collect::<Vec<_>>()
    });

    deltas
        .filter(|d| {
            if d.iter().all(|n| n > &0) {
                // All increasing
                d.iter().all(|n| (1..=3).contains(n))
            } else if d.iter().all(|n| n < &0) {
                // All decreasing
                d.iter().all(|n| (-3..=-1).contains(n))
            } else {
                // Increasing and decreasing
                return false;
            }
        })
        .count()
}

pub fn part2(lines: Vec<String>) -> usize {
    /// Checks if the delta iterable is valid as is or with 1 item removed. Expects the delta iterable
    /// to have at least 3 elements.
    fn is_valid_delta<'a, I: Iterator<Item = &'a isize>>(iter: I) -> bool {
        let delta = iter.cloned().collect::<Vec<_>>();
        assert!(delta.len() >= 3);

        // Check if increasing or decreasing
        let increasing = {
            if delta[0] > 0 && delta[1] > 0 {
                true
            } else if delta[0] < 0 && delta[1] < 0 {
                false
            } else {
                // Index 0 and 1 not the same, use 2nd index as decider
                if delta[2] == 0 {
                    // Direction of delta 0 != delta 1 and delta 2 == 0,
                    // at least 2 modifications needed, invalid
                    return false;
                }
                delta[2] > 0
            }
        };

        let mut modified = false;
        let mut skip_next = false;
        for (i, n) in delta.iter().enumerate() {
            if skip_next {
                skip_next = false;
                continue;
            }

            let range = if increasing { 1..=3 } else { -3..=-1 };

            if !range.contains(n) {
                // See if we can merge this delta with previous or next one
                if modified {
                    // Can't merge more than once
                    return false;
                }

                if i < delta.len() - 1 {
                    let merged = n + delta[i + 1];
                    if range.contains(&merged) {
                        modified = true;
                        skip_next = true;
                        continue;
                    }
                }
                if i >= 1 {
                    let merged = n + delta[i - 1];
                    if range.contains(&merged) {
                        modified = true;
                        continue;
                    }
                }
                if i == 0 {
                    // First item can just be fully removed, causing the first delta to disappear
                    modified = true;
                    continue;
                }
                if i == delta.len() - 1 {
                    // Last item can just be fully removed, causing the last delta to disappear
                    modified = true;
                    continue;
                }
                // Could not merge
                return false;
            }
        }
        true
    }

    let reports = lines.into_iter().map(|s| {
        s.split_whitespace()
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    });

    // Map report [1,3,5,9] to delta array [2,2,4]
    let deltas = reports.map(|report| {
        report
            .windows(2)
            .map(|chunk| chunk[1] as isize - chunk[0] as isize)
            .collect::<Vec<_>>()
    });

    deltas.filter(|d| is_valid_delta(d.iter())).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = [
            "7 6 4 2 1",
            "1 2 7 8 9",
            "9 7 6 2 1",
            "1 3 2 4 5",
            "8 6 4 4 1",
            "1 3 6 7 9",
        ]
        .map(ToOwned::to_owned)
        .to_vec();

        assert_eq!(part1(data), 2)
    }

    #[test]
    fn test_part2() {
        let data = [
            "7 6 4 2 1",
            "1 2 7 8 9",
            "9 7 6 2 1",
            "1 3 2 4 5",
            "8 6 4 4 1",
            "1 3 6 7 9",
        ]
        .map(ToOwned::to_owned)
        .to_vec();

        assert_eq!(part2(data), 4);

        // Additional tests
        // 1 2 6 4: 2 -> 6 too big, 6 -> 4 wrong direction, can remove 6 to have 1 2 4
        // 1 4 -2: removing six becomes 1 2 (4 + -2 = 2)
        assert_eq!(part2(vec!["1 2 6 4".to_owned()]), 1);

        // 1 2 6 7: only 2 -> 6 too big, impossible since would become 1 2 7 (still too big)
        // 1 4 1: removing 6 becomes 1 5 (4 + 1 = 5)
        assert_eq!(part2(vec!["1 2 6 7".to_owned()]), 0);

        // 1 3 2 4: only 3 -> 2 wrong direction, becomes 1 2 4
        // 2 -1 2: removing 2 becomes 2 1 (-1 + 2 = 1)
        assert_eq!(part2(vec!["1 3 2 4".to_owned()]), 1);

        // 1 5 4 3: 1 -> 5 too big, cna remove 1 to have 5 4 3
        // 4 -1 -1: removing 1 becomes -1 -1
        assert_eq!(part2(vec!["1 5 4 3".to_owned()]), 1);

        // 5 9 4 3: 5 -> 9 too big, 9 -> 4 too big, *must* remove 9, not the 5
        // 4 -5 -1: removing 9 becomes -1 -1 (4 + -5 = -1)
        assert_eq!(part2(vec!["5 9 4 3".to_owned()]), 1);

        // 1 4 2 6 7: removing 2 becomes 1 4 6 7
        // 3 -2 4 1: removing 2 becomes 3 2 1
        assert_eq!(part2(vec!["1 4 2 6 7".to_owned()]), 1);

        // 1 5 3 6 7: removing 5 becomes 1 3 6 7
        // 4 -2 3 1: removing 5 becomes 2 3 1
        assert_eq!(part2(vec!["1 5 3 6 7".to_owned()]), 1);

        // 1 3 3 4 6: removing 3 becomes 1 3 4 6
        // 2 0 1 2: removing 3 becomes 2 1 2
        assert_eq!(part2(vec!["1 3 3 4 6".to_owned()]), 1);

        // 3 3 4 6: removing 3 becomes 3 4 6
        // 0 1 2: removing 3 becomes 1 2
        assert_eq!(part2(vec!["3 3 4 6".to_owned()]), 1);

        // 1 2 4 7 12: removing 12 becomes 1 2 4 7
        // 1 2 3 5: removing 12 becomes 1 2 3
        assert_eq!(part2(vec!["1 2 4 7 12".to_owned()]), 1);
    }
}
