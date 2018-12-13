use std::collections::HashSet;
use std::io;
use std::io::Read;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let frequency_changes: Vec<_> = input.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    let mut frequency = 0;
    let mut seen_frequencies = HashSet::new();
    loop {
        for i in &frequency_changes {
            seen_frequencies.insert(frequency);
            frequency += i;
            if seen_frequencies.contains(&frequency) {
                println!("{}", frequency);
                return Ok(());
            }
        }
    }
}
