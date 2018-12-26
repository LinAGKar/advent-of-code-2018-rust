use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut result = String::new();
    for i in input.trim().chars() {
        if let Some(last) = result.pop() {
            if i == last || i.to_ascii_lowercase() != last.to_ascii_lowercase() {
                result.push(last);
                result.push(i);
            }
        } else {
            result.push(i);
        }
    }
    println!("{}", result.len());
}
