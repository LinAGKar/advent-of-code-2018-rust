extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let re = Regex::new(r"(?m)^Step (\w) must be finished before step (\w) can begin\.$").unwrap();
    let mut steps = HashMap::new();
    for i in re.captures_iter(&input) {
        let before = i[1].chars().next().unwrap();
        let after = i[2].chars().next().unwrap();
        steps.entry(after).or_insert(HashSet::new()).insert(before);
        steps.entry(before).or_insert(HashSet::new());
    }
    let mut to_do: HashSet<char> = steps.iter().filter_map(|(&step, dependencies)| {
        if dependencies.is_empty() {
            Some(step)
        } else {
            None
        }
    }).collect();
    for i in &to_do {
        steps.remove(&i);
    }
    let mut order = String::new();
    while !to_do.is_empty() {
        let next = *to_do.iter().min().unwrap();
        to_do.remove(&next);
        order.push(next);
        let mut new = String::new();
        for (&step, dependencies) in &mut steps {
            dependencies.remove(&next);
            if dependencies.is_empty() {
                to_do.insert(step);
                new.push(step);
            }
        }
        for i in new.chars() {
            steps.remove(&i);
        }
    }
    println!("{}", order);
}
