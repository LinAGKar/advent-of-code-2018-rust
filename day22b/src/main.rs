use std::cmp::max;
use std::collections::{BTreeSet, HashMap};
use std::io::Read;

fn get_type(map: &mut Vec<Vec<(u8, u32)>>, depth: u32, x: usize, y: usize, target_x: usize, target_y: usize) -> u8 {
    let old_width = map.len();
    let old_height = if map.is_empty() { 0 } else { map[0].len() };
    let new_width = max(old_width, x + 1);
    let new_height = max(old_height, y + 1);
    map.resize_with(new_width, || vec![(0, 0); new_height]);
    for col in if old_height < new_height { 0 } else { old_width }..new_width {
        let start = if col < old_width {
            map[col].resize(new_height, (0, 0));
            old_height
        } else {
            0
        };
        for row in start..new_height {
            let erosion = (if (col, row) == (target_x, target_y) {
                0
            } else if col == 0 {
                row as u32 * 48271
            } else if row == 0 {
                col as u32 * 16807
            } else {
                map[col - 1][row].1 * map[col][row - 1].1
            } + depth) % 20183;
            map[col][row] = ((erosion % 3) as u8, erosion);
        }
    }

    map[x][y].0
}

fn h(pos: (usize, usize, u8), target: (usize, usize, u8)) -> usize {
    (pos.0 as isize - target.0 as isize).abs() as usize +
    (pos.1 as isize - target.1 as isize).abs() as usize +
    if pos.2 == target.2 { 0 } else { 7 }
}

fn a_star(target: (usize, usize, u8), get_type: &mut dyn FnMut(usize, usize) -> u8) {
    let start = (0, 0, 1);

    let mut g_scores = HashMap::new();
    g_scores.insert(start, 0);

    let mut f_scores = HashMap::new();
    let f_score = h(start, target);
    f_scores.insert(start, f_score);

    let mut open_set = BTreeSet::new();
    open_set.insert((f_score, start));

    while let Some(&next) = open_set.iter().next() {
        open_set.remove(&next);

        let (f_score, pos) = next;
        let (x, y, tool) = pos;
        if pos == target {
            println!("{}", f_score);
            break;
        }

        let g_score = g_scores[&pos];

        for (dx, dy, d_tool) in [
            (2, 1, 0),
            (0, 1, 0),
            (1, 2, 0),
            (1, 0, 0),
            (1, 1, 1),
            (1, 1, 2),
        ] {
            if (x == 0 && dx == 0) || (y == 0 && dy == 0) {
                continue;
            }
            let new_x = x + dx - 1;
            let new_y = y + dy - 1;
            let new_tool = (tool + d_tool) % 3;
            let tentative_g_score = g_score + if d_tool == 0 { 1 } else { 7 };
            let region_type = get_type(new_x, new_y);
            if new_tool == region_type {
                continue;
            }
            let new_pos = (new_x, new_y, new_tool);
            let old_g_score = g_scores.get(&new_pos);
            if tentative_g_score < old_g_score.copied().unwrap_or(usize::MAX) {
                if let Some(_old_g_score) = old_g_score {
                    let old_f_score = f_scores[&new_pos];
                    open_set.remove(&(old_f_score, pos));
                }
                let new_f_score = tentative_g_score + h(new_pos, target);
                open_set.insert((new_f_score, new_pos));
                g_scores.insert(new_pos, tentative_g_score);
                f_scores.insert(new_pos, new_f_score);
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines().map(|line| line.split_whitespace().nth(1).unwrap());
    let depth: u32 = lines.next().unwrap().parse().unwrap();
    let target: Vec<usize> = lines.next().unwrap().split(',').map(|num| num.parse().unwrap()).collect();

    let mut map = Vec::new();

    let mut get_type = |x: usize, y: usize| get_type(&mut map, depth, x, y, target[0], target[1]);
    a_star((target[0], target[1], 1), &mut get_type);
}
