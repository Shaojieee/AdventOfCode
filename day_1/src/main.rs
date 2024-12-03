use std::time::Instant;
use std::collections::BTreeMap;

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



fn p1() -> i64 {
    let input_str = include_str!("./input.txt");
    
    let mut rows: [Vec<i64>; 2] = [vec![], vec![]];
    for (i, num) in input_str.split_whitespace().enumerate() {
        rows[i % 2].push(num.parse().unwrap());
    }

    rows.iter_mut().for_each(|row| row.sort());

    rows[0]
        .iter()
        .zip(&rows[1])
        .map(|(x1, x2)| (x1 - x2).abs())
        .sum()
}


fn p2() -> i64 {
    let mut left: BTreeMap<i64, i64> = BTreeMap::new();
    let mut right: BTreeMap<i64, i64> = BTreeMap::new();

    include_str!("./input.txt")
        .split_whitespace()
        .enumerate()
        .for_each(|(i, x)| {
            let x: i64 = x.parse().unwrap();
            let map = if i % 2 == 0 { &mut right } else { &mut left };
            *map.entry(x).or_insert(0) += 1;
        });

    left.iter().map(|(x,y)| x * y * right.get(x).unwrap_or(&0)).sum()
}
