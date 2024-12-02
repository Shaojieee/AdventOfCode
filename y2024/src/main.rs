use std::collections::BTreeMap;
use std::path::Path;
use std::fs;

fn main() {
    q1_a(Path::new("./y2024/input/day1_a.txt"));
    q1_b(Path::new("./y2024/input/day1_b.txt"));
}

fn read_input(input: &Path) -> String {
    fs::read_to_string(input).expect("Unable to read file")
}


fn q1_a(input: &Path) {
    let input_str = read_input(input);
    
    let mut rows: [Vec<i64>; 2] = [vec![], vec![]];
    for (i, num) in input_str.split_whitespace().enumerate() {
        rows[i % 2].push(num.parse().unwrap());
    }

    rows.iter_mut().for_each(|row| row.sort());

    let result: i64 = rows[0]
        .iter()
        .zip(&rows[1])
        .map(|(x1, x2)| (x1 - x2).abs())
        .sum();

    println!("Result for q1_a: {}", result);
}


fn q1_b(input: &Path) {
    let mut left: BTreeMap<i64, i64> = BTreeMap::new();
    let mut right: BTreeMap<i64, i64> = BTreeMap::new();

    read_input(input)
        .split_whitespace()
        .enumerate()
        .for_each(|(i, x)| {
            let x: i64 = x.parse().unwrap();
            let map = if i % 2 == 0 { &mut right } else { &mut left };
            *map.entry(x).or_insert(0) += 1;
        });

    let results: i64 = left.iter().map(|(x,y)| x * y * right.get(x).unwrap_or(&0)).sum();

    println!("Result for q1_b: {}", results)
}
