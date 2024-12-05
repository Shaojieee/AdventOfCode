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
    let mut update_rules: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    let mut row_iter = input_str.split('\n').into_iter();
    let mut updating: bool = true;
    let mut results: usize = 0;

    while let Some(row) = row_iter.next() {
        if row.is_empty() {
            updating = false;
        }
        else if updating {
            let (num_1, num_2) = row.split_once('|').unwrap();
            update_rules.entry(num_1.parse().unwrap())
            .and_modify(|x| { x.insert(num_2.parse().unwrap()); })
            .or_insert_with(|| BTreeSet::from([num_2.parse().unwrap()]));
        }
        else {
            let mut prev_page: BTreeSet<usize> = BTreeSet::new();
            let mut valid: bool = true;
            let pages: Vec<usize> = row.split(',').map(|x| x.parse().unwrap()).collect();
            for x in pages.iter() {
                if let Some(rules) = update_rules.get(x) {
                    if ! rules.is_disjoint(&prev_page) {
                        valid = false;
                        break;
                    }
                }
                prev_page.insert(*x);
            }
            if valid {
                let mid_index = pages.len() / 2; 
                results += pages[mid_index];
            }
                
        }
    }

    results
}

fn p2() -> usize {
    let input_str = include_str!("./input.txt");
    let mut update_rules: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    let mut row_iter = input_str.split('\n').into_iter();
    let mut updating: bool = true;
    let mut results: usize = 0;

    while let Some(row) = row_iter.next() {
        if row.is_empty() {
            updating = false;
        }
        else if updating {
            let (num_1, num_2) = row.split_once('|').unwrap();
            update_rules.entry(num_1.parse().unwrap())
            .and_modify(|x| { x.insert(num_2.parse().unwrap()); })
            .or_insert_with(|| BTreeSet::from([num_2.parse().unwrap()]));
        }
        else {
            let mut prev_page: BTreeSet<usize> = BTreeSet::new();
            let mut new_page: Vec<usize> = vec![];
            let mut valid: bool = false;
            let mut pages: Vec<usize> = row.split(',').map(|x| x.parse().unwrap()).collect();
            let mut original_valid: bool = true;
            while !valid {
                valid = true;
                for x in 0..pages.len() {
                    if let Some(rules) = update_rules.get(&pages[x]) {
                        if ! rules.is_disjoint(&prev_page) {
                            valid = false;
                            original_valid = false;
                            new_page.insert(0, pages[x]);

                            pages =[new_page,pages[x+1..].to_vec()].concat();
                            new_page = vec![];
                            prev_page = BTreeSet::new();
                            break;
                        } else {
                            prev_page.insert(pages[x]);
                            new_page.push(pages[x]);
                        }
                    }  else {
                        prev_page.insert(pages[x]);
                        new_page.push(pages[x]);
                    }
                }
                
            }
            if !original_valid {
                let mid_index = pages.len() / 2; 
                results += pages[mid_index];
            }
                
        }
    }

    results
}