use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut limits = Vec::new();

    let bots: Vec<_> = input.lines().map(|line| {
        let mut words = line.split_whitespace();
        let pos = words.next().unwrap().split(&['<', '>'][..]).nth(1).unwrap()
                       .split(',').map(|coord| coord.parse::<i32>().unwrap());
        let pos: Vec<_> = pos.collect();
        
        if limits.is_empty() {
            limits.resize(pos.len(), (i32::MAX, i32::MIN));
        }

        let radius = words.next().unwrap().split('=').nth(1).unwrap().parse::<i32>().unwrap();

        for (&pos, limits) in pos.iter().zip(limits.iter_mut()) {
            limits.0 = std::cmp::min(limits.0, pos - radius);
            limits.1 = std::cmp::max(limits.1, pos + radius);
        }

        (pos, radius)
    }).collect();

    let box_size = limits.iter().map(|&(start, end)| end - start + 1).max().unwrap();
    let box_size = (0..).map(|i| 1i32 << i).find(|&i| i >= box_size).unwrap();
    let start_box: Vec<_> = limits.into_iter().map(|(start, end)| start + (end - start + 1) / 2 - box_size / 2).collect();
    let mut boxes_to_check = BinaryHeap::new();
    let subdivisions = 2u8.pow(start_box.len() as u32);
    boxes_to_check.push((bots.len(), Reverse(0), start_box, box_size));

    while let Some((_, _, curr_box, box_size)) = boxes_to_check.pop() {
        if box_size == 1 {
            println!("{}", curr_box.iter().map(|pos| pos.abs()).sum::<i32>());
            break;
        } else {
            let new_box_size = box_size / 2;

            for i in 0..subdivisions {
                let new_box: Vec<_> = curr_box.iter().enumerate().map(|(dim, pos)| pos + if i >> dim & 0b1 != 0 {
                    new_box_size
                } else {
                    0
                }).collect();

                let distance: i32 = new_box.iter().map(|&pos| if 0 < pos {
                    pos
                } else if pos + new_box_size <= 0 {
                    -(pos + new_box_size - 1)
                } else {
                    0
                }).sum();

                let overlapping = bots.iter().filter(|(bot_pos, radius)| {
                    let distance: i32 = new_box.iter().zip(bot_pos).map(|(&pos, &bot_pos)| if bot_pos < pos {
                        pos - bot_pos
                    } else if pos + new_box_size <= bot_pos {
                        bot_pos - (pos + new_box_size - 1)
                    } else {
                        0
                    }).sum();
                    distance <= *radius
                }).count();

                boxes_to_check.push((overlapping, Reverse(distance), new_box, new_box_size));
            }
        }
    }
}
