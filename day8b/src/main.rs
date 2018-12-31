use std::io;
use std::io::Read;

fn parse_node<T>(numbers: &mut T) -> Option<u32> where T:Iterator<Item = u32> {
    let child_count = numbers.next()?;
    let metadata_count = numbers.next()?;
    let mut value = 0;
    if child_count == 0 {
        for _ in 0..metadata_count {
            value += numbers.next()?;
        }
    } else {
        let mut child_values = Vec::new();
        for _ in 0..child_count {
            child_values.push(parse_node(numbers)?);
        }
        for _ in 0..metadata_count {
            let metadata_value = numbers.next()?;
            if 0 < metadata_value && metadata_value <= child_values.len() as u32 {
                value += child_values[(metadata_value - 1) as usize];
            }
        }
    }
    Some(value)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut numbers = input.split_whitespace().map(|x| x.parse::<u32>().unwrap());
    println!("{}", parse_node(&mut numbers).unwrap());
}
