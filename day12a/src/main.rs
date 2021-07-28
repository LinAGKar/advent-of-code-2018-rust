use std::collections::HashMap;
use std::io::{Read,stdin};

fn parse(string: &str) -> Vec<bool> {
    string.chars().map(|chr| chr == '#').collect()
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    const GENS: usize = 20;

    let mut lines = input.lines();
    let init_state = parse(lines.next().unwrap().split_whitespace().nth(2).unwrap());

    let table: HashMap<_, _> = lines.skip(1).map(|line| {
        let mut words = line.split_whitespace();
        let source = parse(words.next().unwrap());
        let target = words.nth(1).unwrap() == "#";
        (source, target)
    }).collect();

    let padding = GENS + 2;
    let state_len = init_state.len() + padding * 2;
    let mut state = vec![false; state_len];
    let mut new_state = vec![false; state_len];

    let mut state_len = init_state.len();
    let mut state_start = padding;

    for (n, i) in init_state.into_iter().enumerate() {
        state[n + padding] = i;
    }

    for _ in 0..GENS {
        for i in state_start - 1..state_start + state_len + 1 {
            new_state[i] = table[&state[i - 2..i + 3]];
        }
        std::mem::swap(&mut state, &mut new_state);
        if state[state_start - 1] {
            state_start -= 1;
            state_len += 1;
        }
        if state[state_start + state_len] {
            state_len += 1;
        }
    }

    println!("{}", (state_start..state_start + state_len).filter(|&i| state[i]).map(|i| (i as i64) - padding as i64).sum::<i64>());
}
