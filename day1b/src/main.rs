use std::collections::VecDeque;
use std::io;
use std::io::Read;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let frequency_changes: Vec<_> = input.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    let mut frequency = 0;
    let mut seen_frequencies: VecDeque<_> = [true].iter().copied().collect();
    let mut start = 0;
    loop {
        for i in &frequency_changes {
            frequency += i;
            while frequency < start {
                seen_frequencies.push_front(false);
                start -= 1;
            }
            let pos = (frequency - start) as usize;
            if pos >= seen_frequencies.len() {
                seen_frequencies.resize(pos + 1, false);
            }
            if seen_frequencies[pos] {
                println!("{}", frequency);
                return Ok(());
            }
            seen_frequencies[pos] = true;
        }
    }
}
