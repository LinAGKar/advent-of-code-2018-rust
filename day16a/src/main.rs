use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let instructions: &[Box<dyn Fn(&mut [usize; 4], usize, usize, usize)>] = &[
        Box::new(|regs: &mut [usize; 4], a: usize, b: usize, c: usize| {
            // addr
            regs[c] = regs[a] + regs[b];
        }),

        Box::new(|regs: &mut [usize; 4], a: usize, b: usize, c: usize| {
            // addi
            regs[c] = regs[a] + b;
        }),

        Box::new(|regs: &mut [usize; 4], a: usize, b: usize, c: usize| {
            // mulr
            regs[c] = regs[a] * regs[b];
        }),

        Box::new(|regs: &mut [usize; 4], a: usize, b: usize, c: usize| {
            // muli
            regs[c] = regs[a] * b;
        }),

        Box::new(|regs: &mut [usize; 4], a: usize, b: usize, c: usize| {
            // banr
            regs[c] = regs[a] & regs[b];
        }),

        Box::new(|regs: &mut [usize; 4], a: usize, b: usize, c: usize| {
            // bani
            regs[c] = regs[a] & b;
        }),

        Box::new(|regs: &mut [usize; 4], a: usize, b: usize, c: usize| {
            // borr
            regs[c] = regs[a] | regs[b];
        }),

        Box::new(|regs: &mut [usize; 4], a: usize, b: usize, c: usize| {
            // bori
            regs[c] = regs[a] | b;
        }),

        Box::new(|regs: &mut [usize; 4], a: usize, _b: usize, c: usize| {
            // setr
            regs[c] = regs[a];
        }),

        Box::new(|regs: &mut [usize; 4], a: usize, _b: usize, c: usize| {
            // seti
            regs[c] = a;
        }),

        Box::new(|regs: &mut [usize; 4], a: usize, b: usize, c: usize| {
            // gtir
            regs[c] = if a > regs[b] { 1 } else { 0 };
        }),

        Box::new(|regs: &mut [usize; 4], a: usize, b: usize, c: usize| {
            // gtri
            regs[c] = if regs[a] > b { 1 } else { 0 };
        }),

        Box::new(|regs: &mut [usize; 4], a: usize, b: usize, c: usize| {
            // gtrr
            regs[c] = if regs[a] > regs[b] { 1 } else { 0 };
        }),

        Box::new(|regs: &mut [usize; 4], a: usize, b: usize, c: usize| {
            // eqir
            regs[c] = if a == regs[b] { 1 } else { 0 };
        }),

        Box::new(|regs: &mut [usize; 4], a: usize, b: usize, c: usize| {
            // eqri
            regs[c] = if regs[a] == b { 1 } else { 0 };
        }),

        Box::new(|regs: &mut [usize; 4], a: usize, b: usize, c: usize| {
            // eqrr
            regs[c] = if regs[a] == regs[b] { 1 } else { 0 };
        }),
    ];

    let re = regex::Regex::new(
        "Before: \\[(\\d+), (\\d+), (\\d+), (\\d+)\\]\\n\
         (\\d+) (\\d+) (\\d+) (\\d+)\\n\
         After:  \\[(\\d+), (\\d+), (\\d+), (\\d+)\\]",
    ).unwrap();

    println!("{}", re.captures_iter(&input).filter(|caps| {
        let regs = [
            caps[1].parse().unwrap(),
            caps[2].parse().unwrap(),
            caps[3].parse().unwrap(),
            caps[4].parse().unwrap(),
        ];
        let regs_after = [
            caps[9].parse().unwrap(),
            caps[10].parse().unwrap(),
            caps[11].parse().unwrap(),
            caps[12].parse().unwrap(),
        ];

        instructions.iter().filter(|instr| {
            let mut regs = regs;
            instr(&mut regs, caps[6].parse().unwrap(), caps[7].parse().unwrap(), caps[8].parse().unwrap());
            regs == regs_after
        }).take(3).count() == 3
    }).count());
}
