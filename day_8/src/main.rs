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
    let mut results: BTreeSet<(i64, i64)> = BTreeSet::new();
    let mut nodes: BTreeMap<char, BTreeSet<(i64, i64)>> = BTreeMap::new();
    let mut row = 0;
    let mut col = 0;
    input_str
    .split('\n')
    .enumerate()
    .for_each(|(r,x)| {
        let r: i64 = r.try_into().unwrap();
        if r > row {
            row = r;
        }
        x
        .chars()
        .enumerate()
        .for_each(|(c, n)| {
            let c: i64 = c.try_into().unwrap();
            if c > col {
                col = c;
            }
            match n {
                '.' => {},
                _ => {
                    nodes.entry(n)
                    .and_modify(|y| {y.insert((r, c));})
                    .or_insert(BTreeSet::from([(r,c)]));
                }
            }
        })

    });

    row += 1;
    col += 1;

    for (_, coors) in nodes {
        let coors:Vec<(i64, i64)> = coors.into_iter().collect();

        for i in 0..coors.len() {
            for j in i+1..coors.len() {
                let a: (i64, i64) = coors.get(i).unwrap().clone().try_into().unwrap();
                let b: (i64, i64) = coors.get(j).unwrap().clone().try_into().unwrap();

                let x_offset = if a.1>b.1 { a.1-b.1 } else { b.1-a.1 };
                let y_offset = if a.0>b.0 { a.0-b.0 } else { b.0-a.0 };

                let mut node_a = (0,0);
                let mut node_b = (0,0);

                node_a.1 = if a.1>b.1 { a.1+x_offset } else { a.1-x_offset };
                node_a.0 = if a.0>b.0 { a.0+y_offset } else { a.0-y_offset };
                node_b.1 = if a.1>b.1 { b.1-x_offset } else { b.1+x_offset };
                node_b.0 = if a.0>b.0 { b.0-y_offset } else { b.0+y_offset };

                if node_a.0 >= 0 && node_a.0 < row && node_a.1 >= 0 && node_a.1 < col {
                    results.insert(node_a);
                }
                if node_b.0 >= 0 && node_b.0 < row && node_b.1 >= 0 && node_b.1 < col {
                    results.insert(node_b);
                }

            }
        }
    }

    results.len()
    
}


fn p2() -> usize {
    let input_str = include_str!("./input.txt");
    let mut results: BTreeSet<(i64, i64)> = BTreeSet::new();
    let mut nodes: BTreeMap<char, BTreeSet<(i64, i64)>> = BTreeMap::new();
    let mut row = 0;
    let mut col = 0;
    input_str
    .split('\n')
    .enumerate()
    .for_each(|(r,x)| {
        let r: i64 = r.try_into().unwrap();
        if r > row {
            row = r;
        }
        x
        .chars()
        .enumerate()
        .for_each(|(c, n)| {
            let c: i64 = c.try_into().unwrap();
            if c > col {
                col = c;
            }
            match n {
                '.' => {},
                _ => {
                    nodes.entry(n)
                    .and_modify(|y| {y.insert((r, c));})
                    .or_insert(BTreeSet::from([(r,c)]));
                }
            }
        })

    });

    row += 1;
    col += 1;

    for (_, coors) in nodes {
        let coors:Vec<(i64, i64)> = coors.into_iter().collect();

        for i in 0..coors.len() {
            for j in i+1..coors.len() {
                let a: (i64, i64) = coors.get(i).unwrap().clone().try_into().unwrap();
                let b: (i64, i64) = coors.get(j).unwrap().clone().try_into().unwrap();

                let x_offset = if a.1>b.1 { a.1-b.1 } else { b.1-a.1 };
                let y_offset = if a.0>b.0 { a.0-b.0 } else { b.0-a.0 };

                let mut node_a = a;
                let mut node_b = b;
                results.insert(node_a);
                results.insert(node_b);
                let mut insertion = true;
                while insertion {
                    insertion = false;
                    node_a.1 = if a.1>b.1 { node_a.1+x_offset } else { node_a.1-x_offset };
                    node_a.0 = if a.0>b.0 { node_a.0+y_offset } else { node_a.0-y_offset };
                    node_b.1 = if a.1>b.1 { node_b.1-x_offset } else { node_b.1+x_offset };
                    node_b.0 = if a.0>b.0 { node_b.0-y_offset } else { node_b.0+y_offset };

                    if node_a.0 >= 0 && node_a.0 < row && node_a.1 >= 0 && node_a.1 < col {
                        results.insert(node_a);
                        insertion = true;
                    }
                    if node_b.0 >= 0 && node_b.0 < row && node_b.1 >= 0 && node_b.1 < col {
                        results.insert(node_b);
                        insertion = true;
                    }
                }
                

            }
        }
    }

    results.len()
    
}
