use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut constants = input.lines().skip(1).map(|line| line.split_whitespace().nth(2).unwrap().parse().unwrap());
    let a: u32 = constants.nth(20).unwrap();
    let b: u32 = constants.next().unwrap();
    let c: u32 = constants.nth(1).unwrap();

    let num = 2 * 2 * 19 * a + b * 22 + c;
    let mut sum = 0;
    for i in 1..=(num as f64).sqrt() as u32 {
        if num % i == 0 {
            let other = num / i;
            sum += i;
            if other != i {
                sum += other;
            }
        }
    }

    println!("{}", sum);
}
