use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut limits = vec![(isize::MAX, isize::MIN); 4];

    let points: Vec<_> = input.lines().map(|line| {
        let coords: Vec<_> = line.split(',').map(|num| num.parse().unwrap()).collect();
        for (&coord, limits) in coords.iter().zip(limits.iter_mut()) {
            limits.0 = std::cmp::min(limits.0, coord);
            limits.1 = std::cmp::max(limits.1, coord);
        }
        coords
    }).collect();

    const R: isize = 3;

    let points: Vec<Vec<_>> = points.into_iter().map(|point| {
        point.into_iter().zip(&limits).map(|(coord, &(min, _))| (coord - min + R) as usize).collect()
    }).collect();

    let sizes: Vec<_> = limits.into_iter().map(|(min, max)| (max - min + 1 + R * 2) as usize).collect();

    let mut map = vec![vec![vec![vec![None; sizes[3]]; sizes[2]]; sizes[1]]; sizes[0]];

    let in_range: Vec<_> = (-R..=R).flat_map(|x| {
        let dist = R - x.abs();
        (-dist..=dist).flat_map(move |y| {
            let dist = R - x.abs() - y.abs();
            (-dist..=dist).flat_map(move |z| {
                let dist = R - x.abs() - y.abs() - z.abs();
                (-dist..=dist).map(move |t| vec![x, y, z, t])
            })
        })
    }).filter(|pos| *pos != [0, 0, 0, 0]).collect();

    let mut constellations: Vec<Vec<Vec<usize>>> = Vec::new();
    let mut neigh_consts = Vec::new();
    let mut new_pos = vec![0; 4];
    let mut tmp = Vec::new();

    for point in points {
        for diff in &in_range {
            for (n, (&pos, &diff)) in point.iter().zip(diff).enumerate() {
                new_pos[n] = (pos as isize + diff) as usize;
            }
            if let Some(constellation) = map[new_pos[0]][new_pos[1]][new_pos[2]][new_pos[3]] {
                if !neigh_consts.contains(&constellation) {
                    neigh_consts.push(constellation);
                }
            }
        }

        let constellation;
        if neigh_consts.is_empty() {
            constellation = constellations.len();
            constellations.push(Vec::new());
        } else {
            constellation = neigh_consts[0]; 
            for &other_const in neigh_consts.iter().skip(1) {
                for other_point in &constellations[other_const] {
                    map[other_point[0]][other_point[1]][other_point[2]][other_point[3]] = Some(constellation);
                }
                std::mem::swap(&mut constellations[other_const], &mut tmp);
                constellations[constellation].extend(tmp.drain(..));
            }
        }
        map[point[0]][point[1]][point[2]][point[3]] = Some(constellation);
        constellations[constellation].push(point);

        neigh_consts.clear();
    }

    println!("{}", constellations.into_iter().filter(|constellation| !constellation.is_empty()).count());
}
