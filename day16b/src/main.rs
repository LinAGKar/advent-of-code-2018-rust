use std::collections::{HashMap,HashSet};
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

    let mut possible_indexes = HashMap::new();

    for caps in re.captures_iter(&input) {
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

        let instr_num: usize = caps[5].parse().unwrap();
        let possible: &mut HashSet<_> = possible_indexes.entry(instr_num).or_insert_with(|| {
            (0..instructions.len()).collect()
        });

        for (n, instr) in instructions.iter().enumerate() {
            let mut regs = regs;
            instr(&mut regs, caps[6].parse().unwrap(), caps[7].parse().unwrap(), caps[8].parse().unwrap());
            if regs != regs_after {
                possible.remove(&n);
            }
        }
    }

    let mut mapping = HashMap::new();

    while let Some((num, index)) = possible_indexes.iter().find_map(|(&num, indexes)| {
        if indexes.len() == 1 {
            Some((num, *indexes.iter().next().unwrap()))
        } else {
            None
        }
    }) {
        mapping.insert(num, index);

        possible_indexes.remove(&num);
        for (_, indexes) in &mut possible_indexes {
            indexes.remove(&index);
        }
    }

    let mut possible_nums = HashMap::new();

    for (&num, indexes) in &possible_indexes {
        for &index in indexes {
            let nums = possible_nums.entry(index).or_insert_with(HashSet::new);
            nums.insert(num);
        }
    }

    while let Some((index, num)) = possible_nums.iter().find_map(|(&index, nums)| {
        if nums.len() == 1 {
            Some((index, *nums.iter().next().unwrap()))
        } else {
            None
        }
    }) {
        mapping.insert(num, index);

        possible_nums.remove(&index);
        for (_, nums) in &mut possible_nums {
            nums.remove(&num);
        }
    }

    let mut lines = input.lines();
    let mut blank_lines = 0;
    while blank_lines < 3 {
        if lines.next().unwrap() == "" {
            blank_lines += 1;
        } else {
            blank_lines = 0;
        }
    }

    let mut regs = [0; 4];

    for line in lines {
        let mut words = line.split_whitespace().map(|word| word.parse::<usize>().unwrap());
        instructions[mapping[&words.next().unwrap()]](
            &mut regs, words.next().unwrap(), words.next().unwrap(), words.next().unwrap(),
        );
    }

    println!("{}", regs[0]);
}
