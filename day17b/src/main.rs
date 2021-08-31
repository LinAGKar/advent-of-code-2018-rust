use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut top = usize::MAX;
    let mut bottom = usize::MIN;
    let mut left = usize::MAX;
    let mut right = usize::MIN;

    let rules: Vec<_> = input.lines().map(|line| {
        let mut ranges: Vec<_> = line.split_whitespace().map(|word| {
            let mut parts = word.trim_matches(',').split('=');
            let dimension = parts.next().unwrap();
            let range = parts.next().unwrap();
            (dimension, if range.contains('.') {
                let mut ends = range.split("..").map(|x| x.parse().unwrap());
                (ends.next().unwrap(), ends.next().unwrap())
            } else {
                let pos = range.parse().unwrap();
                (pos, pos)
            })
        }).collect();
        ranges.sort();
        let x_range = ranges[0].1;
        let y_range = ranges[1].1;
        top = std::cmp::min(y_range.0, top);
        bottom = std::cmp::max(y_range.1, bottom);
        left = std::cmp::min(x_range.0, left);
        right = std::cmp::max(x_range.1, right);
        (x_range, y_range)
    }).collect();

    left -= 1;
    right += 1;

    let mut map = vec![vec!['.'; bottom + 1]; right - left + 1];

    for (x, y) in rules {
        for x in x.0..=x.1 {
            for y in y.0..=y.1 {
                map[x - left][y] = '#';
            }
        }
    }

    let mut to_fill = vec![(500 - left, 0)];
    let mut next = Vec::new();
    let mut settled_count = 0;

    while let Some((x, y)) = to_fill.pop() {
        if ['~', '|'].contains(&map[x][y]) {
            continue;
        }
        map[x][y] = '|';
        if y == bottom {
            continue;
        }
        let under = map[x][y + 1];
        if under == '.' {
            to_fill.push((x, y + 1))
        } else if under == '#' || [1, -1].iter().any(|d| map[(x as isize + d) as usize][y] == '|') {
            for &(x, y) in &[
                (x - 1, y),
                (x + 1, y),
            ] {
                if map[x][y] == '.' {
                    next.push((x, y));
                }
            }
            if next.is_empty() {
                let mut sources = Vec::new();
                for direction in [1, -1] {
                    let mut x = x as isize;
                    while map[x as usize][y] == '|' {
                        if map[x as usize][y - 1] == '|' {
                            sources.push((x, y - 1));
                        }
                        x += direction;
                    }
                    if map[x as usize][y] != '#' {
                        sources.clear();
                        break;
                    }
                }
                if !sources.is_empty() {
                    for d in [1, -1] {
                        let mut x = x as isize;
                        while map[x as usize][y] != '#' {
                            if map[x as usize][y] == '|' {
                                settled_count += 1;
                                map[x as usize][y] = '~';
                            }
                            x += d;
                        }
                    }
                }
                for (x, mut y) in sources {
                    while ![1, -1].iter().any(|d| map[(x as isize + d) as usize][y] == '.') {
                        if map[x as usize][y] == '|' {
                            map[x as usize][y] = '~';
                            settled_count += 1;
                        }
                        y -= 1;
                    }
                    to_fill.extend([1, -1].iter().map(|d| ((x + d) as usize, y)).filter(|&(x, y)| map[x][y] == '.'));
                }
            } else {
                to_fill.extend(&next);
                next.clear();
            }
        } else if under == '~' {
            let mut y = y;
            while ![1, -1].iter().any(|d| map[(x as isize + d) as usize][y] == '.') {
                y -= 1;
            }
            to_fill.extend([1, -1].iter().map(|d| ((x as isize + d) as usize, y)).filter(|&(x, y)| map[x][y] == '.'));
        }
    }

    // for y in 0..=bottom - top {
    //     for x in 0..=right - left {
    //         print!("{}", map[x][y]);
    //     }
    //     println!("");
    // }

    println!("{}", settled_count);
}
