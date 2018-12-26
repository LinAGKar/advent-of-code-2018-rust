#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::io::Read;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
enum EntryType {
    FallAsleep,
    WakeUp,
    BeginShift(i32),
}


#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
struct LogEntry {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    entry_type: EntryType,
}

impl LogEntry {
    fn new(entry: &str) -> LogEntry {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\[(\d+)-(\d+)-(\d+) (\d+):(\d+)\] (Guard #(\d+)|f|w)").unwrap();
        }
        let entry_match = RE.captures(entry).unwrap();
        LogEntry {
            year: entry_match.get(1).unwrap().as_str().parse().unwrap(),
            month: entry_match.get(2).unwrap().as_str().parse().unwrap(),
            day: entry_match.get(3).unwrap().as_str().parse().unwrap(),
            hour: entry_match.get(4).unwrap().as_str().parse().unwrap(),
            minute: entry_match.get(5).unwrap().as_str().parse().unwrap(),
            entry_type: if let Some(guard_id) = entry_match.get(7) {
                EntryType::BeginShift(guard_id.as_str().parse().unwrap())
            } else if entry_match.get(6).unwrap().as_str() == "f" {
                EntryType::FallAsleep
            } else {
                EntryType::WakeUp
            },
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut log: Vec<LogEntry> = input.lines().map(|x| LogEntry::new(x)).collect();
    log.sort_unstable();
    let mut guard_times = HashMap::new();
    let mut active_guard = 0;
    let mut fall_asleep_minute = 0;
    for i in log {
        match i.entry_type {
            EntryType::FallAsleep => {
                fall_asleep_minute = i.minute;
            }

            EntryType::WakeUp => {
                let time = guard_times.entry(active_guard).or_insert(vec![0; 60]);
                for j in fall_asleep_minute..i.minute {
                    (*time)[j as usize] += 1;
                }
            }

            EntryType::BeginShift(id) => {
                active_guard = id;
            }
        }
    }
    let (&id, minute) = guard_times
        .iter()
        .max_by_key(|x| x.1.iter().sum::<i32>())
        .map(|x| (x.0, x.1
            .iter()
            .enumerate()
            .max_by_key(|&(_, days)| days)
            .map(|(minute, _)| minute as i32)
            .unwrap()
        ))
        .unwrap();
    println!("{}", id * minute);
}
