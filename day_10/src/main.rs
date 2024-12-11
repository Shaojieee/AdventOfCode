use std::{collections::{BTreeMap, BTreeSet}, time::Instant};

fn main() {
    
    let now = Instant::now();
    let r1 = p1();
    let elapsed = now.elapsed();
    println!("Result for p1: {}", r1);
    println!("part1 : {:.2?}", elapsed);

    let now = Instant::now();
    let r2 = p2();
    let elapsed = now.elapsed();
    println!("Result for p2: {}", r2);
    println!("part1 : {:.2?}", elapsed);
}

fn p1() -> usize {
    let input_str = include_str!("./input.txt");
    let mut results: usize = 0;
    let mut map: Vec<Vec<usize>> = vec![];
    let mut trial_head: BTreeSet<(usize, usize)> = BTreeSet::new();

    input_str
    .split('\n')
    .enumerate()
    .for_each(|(r, x)| {
        map.push(vec![]);
        x
        .chars()
        .enumerate()
        .for_each(|(c, y)| {
            let level = y.to_digit(10).unwrap_or(11) as usize;
            if level == 0 {
                trial_head.insert((r,c));
            }
            map.get_mut(r).unwrap().push(level);
        })
    });

    for (r,c) in trial_head {
        let r = r as isize; 
        let c = c as isize;
        results += p1_path_finder(r, c, 0, map.as_ref()).len();
    }

    results
}

fn p1_path_finder(r: isize, c: isize, lvl: usize, map: &Vec<Vec<usize>>) -> BTreeSet<(isize, isize)> {
    if lvl == 9 {
        return BTreeSet::from([(r,c)]);
    }
    let map_r = map.len() as isize;
    let map_c = map[0].len() as isize;
    let mut results = BTreeSet::new();

    let directions = vec![(-1,0), (0,1), (1,0), (0,-1)];
    for (y,x) in directions {
        let r_new = r + y;
        let c_new = c + x;
        if r_new >= 0 && r_new < map_r && c_new >= 0 && c_new < map_c && map[r_new as usize][c_new as usize] == lvl + 1 {
            results.extend(p1_path_finder(r_new, c_new, lvl+1, map));
        }
    }
    results
}


fn p2() -> usize {
    let input_str = include_str!("./input.txt");
    let mut results: usize = 0;
    let mut map: Vec<Vec<usize>> = vec![];
    let mut trial_head: BTreeSet<(usize, usize)> = BTreeSet::new();

    input_str
    .split('\n')
    .enumerate()
    .for_each(|(r, x)| {
        map.push(vec![]);
        x
        .chars()
        .enumerate()
        .for_each(|(c, y)| {
            let level = y.to_digit(10).unwrap_or(11) as usize;
            if level == 0 {
                trial_head.insert((r,c));
            }
            map.get_mut(r).unwrap().push(level);
        })
    });

    for (r,c) in trial_head {
        let r = r as isize; 
        let c = c as isize;
        results += p2_path_finder(r, c, 0, map.as_ref()).values().sum::<usize>();
    }

    results
}

fn p2_path_finder(r: isize, c: isize, lvl: usize, map: &Vec<Vec<usize>>) -> BTreeMap<(isize, isize), usize> {
    if lvl == 9 {
        return BTreeMap::from([((r,c),1)]);
    }
    let map_r = map.len() as isize;
    let map_c = map[0].len() as isize;
    let mut results = BTreeMap::new();

    let directions = vec![(-1,0), (0,1), (1,0), (0,-1)];
    for (y,x) in directions {
        let r_new = r + y;
        let c_new = c + x;
        if r_new >= 0 && r_new < map_r && c_new >= 0 && c_new < map_c && map[r_new as usize][c_new as usize] == lvl + 1 {
            p2_path_finder(r_new, c_new, lvl+1, map)
            .into_iter()
            .for_each(|((r,c), x)| {
                results.entry((r, c)).and_modify(|y| {*y+=x;}).or_insert(x);
            })
        }
    }
    results
}
