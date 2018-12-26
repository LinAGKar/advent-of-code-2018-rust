use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("{}", "qwertyuiopasdfghjklzxcvbnm".chars().map(|x| {
        let mut result = String::new();
        for i in input.trim().chars().filter(|y| y.to_ascii_lowercase() != x) {
            if let Some(last) = result.pop() {
                if i == last || i.to_ascii_lowercase() != last.to_ascii_lowercase() {
                    result.push(last);
                    result.push(i);
                }
            } else {
                result.push(i);
            }
        }
        result.len()
    }).min().unwrap());
}
