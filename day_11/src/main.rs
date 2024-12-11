use std::{collections::BTreeMap, time::Instant};

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

    input_str
    .split(' ')
    .for_each(|x| {
        let mut x = vec![(*x.trim()).to_string()];
        for _ in 0..25 {
            for i in 0..x.len() {
                if x[i] == "0"{
                    x[i] = "1".to_string();
                } else if x[i].len() % 2 == 0 {
                    let new_len = x[i].len() / 2;
                    let str_1 = x[i][..new_len].trim_start_matches("0").to_string();
                    let str_2 = x[i][new_len..].trim_start_matches("0").to_string();
                    if str_1.is_empty() {
                        x.push("0".to_string());
                    } else {
                        x.push(str_1);
                    }

                    if str_2.is_empty() {
                        x[i] = "0".to_string();
                    } else {
                        x[i] = str_2;
                    }
                } else {
                    x[i] = (x[i].parse::<i64>().expect(&format!("Invalid digit: {}", x[i])) * 2024).to_string();
                }
            }
        }
        results += x.len();
    });


    results
}


fn p2() -> usize {
    let input_str = include_str!("./input.txt");
    let mut results: usize = 0;
    let mut numbers_map: BTreeMap<(String, usize), usize> = BTreeMap::new();
    input_str
    .split(' ')
    .for_each(|x| {
        numbers_map.entry(((*x).trim().to_string(), 75)).and_modify(|f| {*f += 1;}).or_insert(1);
    });


    let mut lookup: BTreeMap<(String, usize), usize> = BTreeMap::new();
    let keys = numbers_map.keys().cloned();
    for (x, s) in  keys {
        p2_split(x, 75, &mut lookup);
    }

    for k in numbers_map.into_keys() {
        results += lookup.get(&k).expect(&format!("Unable to find {k:?}"));
    }

    results
}

fn p2_split(x: String, max_splits: usize, lookup: &mut BTreeMap<(String, usize), usize>) {
    if max_splits == 0 {
        lookup.insert((x.clone(),0),1);
    }

    if lookup.contains_key(&(x.clone(), max_splits)) {
        return;
    }

    if x == "0"{
        p2_split("1".to_string(), max_splits-1, lookup);
        let curr_results = *(lookup.get(&("1".to_string(), max_splits-1)).unwrap());
        lookup.insert((x, max_splits), curr_results);

    } else if x.len() % 2 == 0 {


        let new_len = x.len() / 2;
        let str_1 = x[..new_len].trim_start_matches("0").to_string();
        let str_2 = x[new_len..].trim_start_matches("0").to_string();
        let mut left_results = 0;
        if str_1.is_empty() {
            left_results = match lookup.contains_key(&("0".to_string(), max_splits-1)) {
                true => *lookup.get(&("0".to_string(), max_splits-1)).unwrap(),
                false => {
                    p2_split("0".to_string(), max_splits-1, lookup);
                    *lookup.get(&("0".to_string(), max_splits-1)).unwrap()
                }
            };
        } else {
            left_results = match lookup.contains_key(&(str_1.clone(), max_splits-1)) {
                true => *lookup.get(&(str_1.clone(), max_splits-1)).unwrap(),
                false => {
                    p2_split(str_1.clone(), max_splits-1, lookup);
                    *lookup.get(&(str_1.clone(), max_splits-1)).unwrap()
                }
            };
        }

        let mut right_results = 0;
        if str_2.is_empty() {
            right_results = match lookup.contains_key(&("0".to_string(), max_splits-1)) {
                true => *lookup.get(&("0".to_string(), max_splits-1)).unwrap(),
                false => {
                    p2_split("0".to_string(), max_splits-1, lookup);
                    *lookup.get(&("0".to_string(), max_splits-1)).unwrap()
                }
            };
        } else {
            right_results = match lookup.contains_key(&(str_2.clone(), max_splits-1)) {
                true => *lookup.get(&(str_2.clone(), max_splits-1)).unwrap(),
                false => {
                    p2_split(str_2.clone(), max_splits-1, lookup);
                    *lookup.get(&(str_2.clone(), max_splits-1)).unwrap()
                }
            };
        }

        lookup.insert((x, max_splits), left_results + right_results);

    } else {
        let new_str = (x.parse::<i64>().expect(&format!("Invalid digit: {}", x)) * 2024).to_string();
        p2_split(new_str.clone(), max_splits-1, lookup);
        
        let curr_results = *(lookup.get(&(new_str, max_splits-1)).unwrap());
        lookup.insert((x, max_splits), curr_results);
    }

}

