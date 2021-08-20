use std::collections::{BTreeSet,HashMap};
use std::io::{Read,stdin};
use std::ops::{Add,Sub};
use std::convert::TryInto;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Mob {
    Goblin,
    Elf,
}

#[derive(PartialEq, Clone, Copy, Eq, Hash, PartialOrd, Ord, Debug)]
struct Vec2<T> {
    y: T,
    x: T,
}

impl<T: Add> Add<(T, T)> for Vec2<T> where
    <T as Add>::Output: Into<T> {
    type Output = Vec2<T>;

    fn add(self, other: (T, T)) -> Vec2<T> {
        Vec2 {
            y: (self.y + other.0).into(),
            x: (self.x + other.1).into(),
        }
    }
}

impl<T> Vec2<T>{
    fn cast<U>(self) -> Vec2<U> where
        T: TryInto<U>,
        <T as TryInto<U>>::Error: std::fmt::Debug {
        Vec2 {
            y: self.y.try_into().unwrap(),
            x: self.x.try_into().unwrap(),
        }
    }

    fn manhattan(self) -> T where
        T: num_traits::Signed {
        self.y.abs() + self.x.abs()
    }
}

impl<T: Sub> Sub for Vec2<T> where
    <T as Sub>::Output: Into<T> {
    type Output = Vec2<T>;

    fn sub(self, other: Vec2<T>) -> Vec2<T> {
        Vec2 {
            y: (self.y - other.y).into(),
            x: (self.x - other.x).into(),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Wall,
    Mob,
    Air,
}

fn h(pos: Vec2<i8>, goal: Vec2<i8>) -> i8 {
    (goal - pos).manhattan()
}

fn a_star(start: Vec2<i8>, goal: Vec2<i8>, map: &Vec<Vec<Tile>>) -> Option<(i8, HashMap<Vec2<i8>, Vec2<i8>>)> {
    let mut g_scores = HashMap::new();
    g_scores.insert(start, 0);

    let f_score = h(start, goal);
    let mut f_scores = HashMap::new();
    f_scores.insert(start, f_score);

    let mut open_set = BTreeSet::new();
    open_set.insert((f_score, start));

    let mut came_from = HashMap::new();

    while let Some(&curr) = open_set.iter().next() {
        let (f_score, pos) = curr;

        if pos == goal {
            return Some((f_score, came_from));
        }

        open_set.remove(&curr);
        let g_score = g_scores[&pos];

        for new_pos in [
            (-1, 0),
            (0, -1),
            (0, 1),
            (1, 0),
        ].iter().map(|&delta| pos + delta).filter(|&new_pos| {
            map[new_pos.y as usize][new_pos.x as usize] == Tile::Air || new_pos == goal
        }) {
            let tentative_g_score = g_score + 1;
            let old_g_score = g_scores.get(&new_pos);
            if tentative_g_score < old_g_score.copied().unwrap_or(i8::MAX) {
                came_from.insert(new_pos, pos);
                if let Some(&old_f_score) = f_scores.get(&new_pos) {
                    open_set.remove(&(old_f_score, new_pos));
                }

                g_scores.insert(new_pos, tentative_g_score);
                let new_f_score = tentative_g_score + h(new_pos, goal);
                f_scores.insert(new_pos, new_f_score);
                open_set.insert((new_f_score, new_pos));
            }
        }
    }

    None
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    const ATK: u8 = 3;
    const HP: u8 = 200;

    let mut mobs = HashMap::new();
    let mut mobs_iter = Vec::new();

    let mut map: Vec<Vec<_>> = input.lines().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, chr)| match chr {
            '#' => Tile::Wall,
            '.' => Tile::Air,
            'E' => {
                let pos = Vec2{y: y, x: x};
                mobs.insert(pos, (HP, Mob::Elf, -1));
                mobs_iter.push(pos);
                Tile::Mob
            }
            'G' => {
                let pos = Vec2{y: y, x: x};
                mobs.insert(pos, (HP, Mob::Goblin, -1));
                mobs_iter.push(pos);
                Tile::Mob
            }
            _ => panic!(""),
        }).collect()
    }).collect();

    let mut new_mobs = Vec::new();

    'round: for round in 0.. {
        for (n, &pos) in mobs_iter.iter().enumerate() {
            let mob = if let Some((_, mob, moved_round)) = mobs.get_mut(&pos) {
                if *moved_round >= round as i16 {
                    continue;
                }
                *moved_round = round as i16;
                *mob
            } else {
                continue;
            };

            let pos = if [
                (0, 1),
                (1, 0),
                (1, 2),
                (2, 1),
            ].iter().any(|&diff| {
                if let Some(&(_, other_mob, _)) = mobs.get(&(pos - Vec2{y: 1, x: 1} + diff)) {
                    other_mob != mob
                } else {
                    false
                }
            }) {
                pos
            } else {
                let distances: Vec<_> = mobs.iter().filter_map(|(&other_pos, &(_, other_mob, _))| {
                    if other_mob != mob {
                        a_star(
                            pos.cast(), other_pos.cast(), &map,
                        ).map(|(dist, came_from)| (other_pos, dist, came_from))
                    } else {
                        None
                    }
                }).collect();

                if let Some(closest) = distances.iter().map(|&(_, dist, _)| dist).min() {
                    let targets: Vec<_> = distances.into_iter().filter(|&(_, dist, _)| dist == closest).collect();
                    let first = targets.iter().map(|&(pos, _, _)| pos).min().unwrap();
                    let (target, dist, came_from) = targets.into_iter().filter(|&(enemy_pos, _, _)| {
                        enemy_pos < first + (2, 0)
                    }).flat_map(|(enemy_pos, dist, came_from)| {
                        let came_from_enemy: Vec2<usize> = came_from[&enemy_pos.cast()].cast();
                        [
                            (0, 1),
                            (1, 0),
                            (1, 2),
                            (2, 1),
                        ]
                            .iter()
                            .map(move |&diff| enemy_pos - Vec2{y: 1, x: 1} + diff)
                            .filter(|tgt_pos| map[tgt_pos.y][tgt_pos.x] == Tile::Air)
                            .take_while(move |&tgt_pos| tgt_pos != came_from_enemy)
                            .filter_map(|tgt_pos| {
                                a_star(pos.cast(), tgt_pos.cast(), &map)
                                    .map(|(dist, came_from)| (tgt_pos, dist, came_from))
                            })
                            .chain([(came_from_enemy, dist - 1, came_from)])
                    }).min_by_key(|&(tgt_pos, dist, _)| (dist, tgt_pos)).unwrap();

                    let potential_pos = (0..).try_fold(target.cast(), |step, _| {
                        let prev = came_from[&step];
                        if prev == pos.cast() {
                            Err(step)
                        } else {
                            Ok(prev)
                        }
                    }).unwrap_err();

                    let new_pos = [
                        (0, 1),
                        (1, 0),
                        (1, 2),
                        (2, 1),
                    ]
                        .iter()
                        .map(|&diff| pos - Vec2{y: 1, x: 1} + diff)
                        .filter(|neighbor| map[neighbor.y][neighbor.x] == Tile::Air)
                        .find(|&neighbor| {
                            let neighbor = neighbor.cast();
                            neighbor == potential_pos || 
                                a_star(neighbor, target.cast(), &map)
                                    .map(|(neigh_dist, _)| neigh_dist == dist - 1)
                                    .unwrap_or(false)
                        }).unwrap();

                    let mob = mobs.remove(&pos).unwrap();
                    map[pos.y][pos.x] = Tile::Air;
                    mobs.insert(new_pos, mob);
                    map[new_pos.y][new_pos.x] = Tile::Mob;
                    new_pos
                } else {
                    pos
                }
            };

            new_mobs.push(pos);

            if let Some((_, pos)) = [
                (0, 1),
                (1, 0),
                (1, 2),
                (2, 1),
            ].iter().filter_map(|&diff| {
                let pos = pos - Vec2{y: 1, x: 1} + diff;
                mobs.get(&pos).filter(|&&(_, other_mob, _)| other_mob != mob).map(|&(hp, _, _)| (hp, pos))
            }).min() {
                let (hp, _, _) = mobs.get_mut(&pos).unwrap();

                if *hp <= ATK {
                    map[pos.y][pos.x] = Tile::Air;
                    mobs.remove(&pos);
                } else {
                    *hp -= ATK;
                }
            }

            let first_mob_type = mobs.values().next().unwrap().1;
            if mobs.values().all(|&(_, mob_type, _)| mob_type == first_mob_type) {
                let hitpoints: u32 = mobs.values().map(|&(hp, _, _)| hp as u32).sum();
                let completed_rounds = round + if mobs_iter.iter().skip(n + 1).any(|pos| mobs.contains_key(pos)) {
                    0
                } else {
                    1
                };
                println!("{}", completed_rounds * hitpoints);
                break 'round;
            }
        }

        std::mem::swap(&mut mobs_iter, &mut new_mobs);
        new_mobs.clear();
        mobs_iter.sort();
    }
}
