use std::collections::HashMap;
use std::io::{Read,stdin};

fn parse(string: &str) -> Vec<bool> {
    string.chars().map(|chr| chr == '#').collect()
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    const GENS: usize = 50000000000;

    let mut lines = input.lines();
    let init_state = parse(lines.next().unwrap().split_whitespace().nth(2).unwrap());

    let table: HashMap<_, _> = lines.skip(1).map(|line| {
        let mut words = line.split_whitespace();
        let source = parse(words.next().unwrap());
        let target = words.nth(1).unwrap() == "#";
        (source, target)
    }).collect();

    let mut padding = 10;
    let state_len = init_state.len() + padding * 2;
    let mut state = vec![false; state_len];
    let mut new_state = vec![false; state_len];

    let mut state_len = init_state.len();
    let mut state_start = padding;

    for (n, i) in init_state.into_iter().enumerate() {
        state[n + padding] = i;
    }

    for gen in 0..GENS {
        if state_start <= 2 || state_start + state_len >= state.len() - 2 {
            let new_padding = padding * 2;
            new_state = vec![false; state.len() + padding * 2];
            for (n, i) in state.into_iter().enumerate() {
                new_state[n + padding] = i;
            }
            state = new_state;
            new_state = vec![false; state.len()];
            state_start += padding;
            padding = new_padding;
        }

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

        if (state_start..state_start + state_len).all(|i| {
            !state[i] || [0, 1, 3, 4].iter().all(|&j| !state[i - 2 + j])
        }) {
            let curr = (state_start..state_start + state_len).filter(|&i| state[i]).map(|i| (i as isize) - padding as isize).sum::<isize>();
            let count = (state_start..state_start + state_len).filter(|&i| state[i]).count();
            println!("{}", curr + ((GENS - gen - 1) * count) as isize);
            break;
        }
    }
}
