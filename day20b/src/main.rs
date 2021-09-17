use std::collections::{HashMap, HashSet, VecDeque};
use std::io::Read;

#[derive(Debug)]
enum Token {
    N,
    S,
    W,
    E,
    Options(Vec<Vec<Token>>),
}

fn parse_path(path: &[u8], mut pos: usize) -> (Vec<Vec<Token>>, usize) {
    let mut options = Vec::new();
    let mut curr_option = Vec::new();

    while let Some(&curr) = path.get(pos) {
        pos += 1;

        match curr as char {
            '|' => {
                options.push(curr_option);
                curr_option = Vec::new();
            }

            'N' => { curr_option.push(Token::N); }
            'S' => { curr_option.push(Token::S); }
            'W' => { curr_option.push(Token::W); }
            'E' => { curr_option.push(Token::E); }
            ')' => { break; }
            '(' => {
                let (options, new_pos) = parse_path(path, pos);
                curr_option.push(Token::Options(options));
                pos = new_pos;
            }
            _ => panic!(),
        }
    }

    options.push(curr_option);
    (options, pos)
}

fn walk_path(start: Vec<(i16, i16)>, path: &Vec<Token>, map: &mut HashMap<(i16, i16), u8>) -> Vec<(i16, i16)> {
    let mut coords = start;

    for token in path {
        match token {
            Token::N => {
                for i in &mut coords {
                    *map.entry(*i).or_insert(0) |= 0b0001;
                    i.1 += 1;
                    *map.entry(*i).or_insert(0) |= 0b0010;
                }
            }

            Token::S => {
                for i in &mut coords {
                    *map.entry(*i).or_insert(0) |= 0b0010;
                    i.1 -= 1;
                    *map.entry(*i).or_insert(0) |= 0b0001;
                }
            }

            Token::W => {
                for i in &mut coords {
                    *map.entry(*i).or_insert(0) |= 0b0100;
                    i.0 -= 1;
                    *map.entry(*i).or_insert(0) |= 0b1000;
                }
            }

            Token::E => {
                for i in &mut coords {
                    *map.entry(*i).or_insert(0) |= 0b1000;
                    i.0 += 1;
                    *map.entry(*i).or_insert(0) |= 0b0100;
                }
            }

            Token::Options(options) => {
                let mut new_coords = Vec::new();
                for option in options {
                    for coord in walk_path(coords.clone(), option, map) {
                        if !new_coords.contains(&coord) {
                            new_coords.push(coord);
                        }
                    }
                }
                coords = new_coords;
            }
        }
    }

    coords
}

fn main() {
    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input).unwrap();

    let (end, _) = input.iter().enumerate().find(|&(_, &character)| character == '$' as u8).unwrap();
    let path = &input[1..end];

    let (options, _) = parse_path(&path, 0);
    assert_eq!(options.len(), 1);
    let mut map = HashMap::new();
    walk_path(vec![(0, 0)], &options[0], &mut map);

    let mut visited_rooms = HashSet::new();
    let mut next_rooms: VecDeque<_> = [((0, 0), 0)].iter().cloned().collect();
    let mut count = 0;

    while let Some((room, distance)) = next_rooms.pop_front() {
        if visited_rooms.contains(&room) {
            continue;
        }
        if distance >= 1000 {
            count += 1;
        }
        visited_rooms.insert(room);
        let doors = map[&room];
        for (dx, dy, door) in [
            (0, 1, 0b0001),
            (0, -1, 0b0010),
            (-1, 0, 0b0100),
            (1, 0, 0b1000),
        ] {
            if doors & door != 0 {
                let next_room = (room.0 + dx, room.1 + dy);
                next_rooms.push_back((next_room, distance + 1));
            }
        }
    }

    println!("{}", count);
}
