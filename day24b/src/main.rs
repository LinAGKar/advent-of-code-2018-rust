use std::collections::{BinaryHeap, HashSet};
use std::io::Read;

#[derive(Clone)]
struct Group<'a> {
    count: i32,
    hp: i32,
    immunities: HashSet<&'a str>,
    weaknesses: HashSet<&'a str>,
    atk: i32,
    damage_type: &'a str,
    initiative: u8,
    target: bool,
}

fn run_battle(groups: &Vec<Vec<Group>>, boost: i32) -> (usize, i32) {
    let mut groups = groups.clone();

    for group in &mut groups[0] {
        group.atk += boost;
    }

    let mut attackers = BinaryHeap::new();
    let mut attacks = BinaryHeap::new();
    let mut available_targets = HashSet::new();

    while !groups.iter().any(|groups| groups.is_empty()) {
        attackers.extend(groups.iter().enumerate().flat_map(|(side, groups)| {
            groups.iter().enumerate().map(move |(index, group)| {
                (group.count * group.atk, group.initiative, side, index)
            })
        }));

        available_targets.clear();
        available_targets.extend((0..groups.len()).flat_map(|side| {
            (0..groups[side].len()).map(move |index| (side, index))
        }));

        while let Some((_, _, side, index)) = attackers.pop() {
            let group = &groups[side][index];
            let power = group.count * group.atk;
            if let Some(&(tgt_side, tgt_index)) = available_targets.iter().filter(|&&(tgt_side, tgt_index)| {
                side != tgt_side && !groups[tgt_side][tgt_index].immunities.contains(group.damage_type)
            }).max_by_key(|&&(tgt_side, tgt_index)| {
                let target_group = &groups[tgt_side][tgt_index];

                let dmg = if target_group.weaknesses.contains(group.damage_type) {
                    power * 2
                } else {
                    power
                };
                (dmg, target_group.count * target_group.atk, target_group.initiative)
            }) {
                available_targets.remove(&(tgt_side, tgt_index));
                attacks.push((group.initiative, side, index, tgt_side, tgt_index));
            }
        }

        let mut did_damage = false;

        while let Some((_, atk_side, atk_index, tgt_side, tgt_index)) = attacks.pop() {
            let attacker = &groups[atk_side][atk_index];
            if attacker.count > 0 {
                let damage_type = attacker.damage_type;
                let power = attacker.count * attacker.atk;
                let target = &mut groups[tgt_side][tgt_index];
                let dmg = if target.weaknesses.contains(&damage_type) {
                    power * 2
                } else {
                    power
                } / target.hp;
                if dmg > 0 {
                    did_damage = true;
                }
                target.count -= dmg;
            }
        }

        if !did_damage {
            return (1, 0);
        }

        for groups in &mut groups {
            let len = groups.len();
            for i in 0..len {
                let index = len - i - 1;
                if groups[index].count <= 0 {
                    groups.remove(index);
                }
            }
        }
    }

    let total_count = groups.iter().flat_map(|groups| groups.iter().map(|group| group.count)).sum::<i32>();
    let winner = groups.iter().enumerate().find_map(|(side, groups)| if !groups.is_empty() {
        Some(side)
    } else {
        None
    }).unwrap();

    (winner, total_count)
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let group_re = regex::Regex::new(
        "^(\\d+) units each with (\\d+) hit points(?: \\(([\\w, ;]+)\\))? \
        with an attack that does (\\d+) (\\w+) damage at initiative (\\d+)$",
    ).unwrap();
    let modifier_re = regex::Regex::new(r"(immune|weak) to ([\w, ]+)").unwrap();

    let mut groups = vec![Vec::new(); 2];
    let mut side = 0;

    for line in input.lines() {
        if line == "Infection:" {
            side = 1;
        }

        if let Some(caps) = group_re.captures(line) {
            let count: i32 = caps[1].parse().unwrap();
            let hp: i32 = caps[2].parse().unwrap();
            let mut weaknesses = HashSet::new();
            let mut immunities = HashSet::new();
            if let Some(cap) = caps.get(3) {
                for list in modifier_re.captures_iter(cap.as_str()) {
                    (if &list[1] == "immune" { &mut immunities } else { &mut weaknesses })
                        .extend(list.get(2).unwrap().as_str().split(", "));
                }
            }
            let damage: i32 = caps[4].parse().unwrap();
            let damage_type = caps.get(5).unwrap().as_str();
            let initiative = caps[6].parse().unwrap();
            groups[side].push(Group {
                count: count,
                hp: hp,
                weaknesses: weaknesses,
                immunities: immunities,
                atk: damage,
                damage_type: damage_type,
                initiative: initiative,
                target: true,
            });
        }
    }

    let mut low = 0;
    let mut high = 1;
    let mut high_count;

    loop {
        let (winner, count) = run_battle(&groups, high);
        if winner == 0 {
            high_count = count;
            break;
        } else {
            low = high;
            high *= 2;
        }
    }

    while high > low + 1 {
        let middle = low + (high - low) / 2;
        let (winner, count) = run_battle(&groups, middle);
        if winner == 0 {
            high = middle;
            high_count = count;
        } else {
            low = middle;
        }
    }

    println!("{}", high_count);
}
