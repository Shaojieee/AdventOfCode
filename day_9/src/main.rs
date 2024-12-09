use std::time::Instant;

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
    let mut curr_pos: usize = 0;
    let mut curr_id: usize = 0;
    let mut empty: Vec<usize> = vec![];

    let disk_map: Vec<usize> = input_str.chars().map(|x| x.to_digit(10).unwrap() as usize).collect();
    let mut disk_map_iter = disk_map.iter().enumerate();

    while let Some((i, x)) = disk_map_iter.next() {
        if i % 2 == 0 {
            results += curr_id * (x * curr_pos + (x*(x-1))/2);

            curr_id += 1;
            curr_pos += x;
        } else {
            empty = [empty,(curr_pos..curr_pos+x).collect()].concat();
            curr_pos += x;
        }
    }

    let mut max_id = curr_id - 1;
    let mut max_pos = curr_pos - 1;

    let remainder = disk_map.len() % 2;
    let mut disk_map_iter = disk_map.into_iter().rev().enumerate();

    while let Some((i, x)) = disk_map_iter.next() {
        if empty.is_empty() || *empty.first().unwrap() >= max_pos {
            break;
        }
        if i % 2 != remainder {
            let mut y = x;
            while y > 0 && !empty.is_empty() {
                if *empty.first().unwrap() >= max_pos {
                    break;
                }
                let pos = empty.remove(0);
                let to_add = pos * max_id;
                let to_remove = max_pos * max_id;
                results += to_add;
                results -= to_remove;
                max_pos -= 1;
                y -= 1;
            }
            max_id -= 1;
        } else {
            max_pos -= x;
        }
    }

    results
    
}


fn p2() -> usize {
    let input_str = include_str!("./input.txt");
    let mut results: usize = 0;
    let mut curr_pos: usize = 0;
    let mut curr_id: usize = 0;
    let mut empty: Vec<(usize, usize)> = vec![];

    let disk_map: Vec<usize> = input_str.chars().map(|x| x.to_digit(10).unwrap() as usize).collect();
    let mut disk_map_iter = disk_map.iter().enumerate();

    while let Some((i, x)) = disk_map_iter.next() {
        if i % 2 == 0 {
            results += curr_id * (x * curr_pos + (x*(x-1))/2);

            curr_id += 1;
            curr_pos += x;
        } else {
            empty.push((curr_pos, *x));
            curr_pos += x;
        }
    }

    let mut max_id = curr_id - 1;
    let mut max_pos = curr_pos - 1;

    let remainder = disk_map.len() % 2;
    let mut disk_map_iter = disk_map.into_iter().rev().enumerate();

    while let Some((i, x)) = disk_map_iter.next() {
        let (pos, size) = empty.first().unwrap();
        if empty.is_empty() ||  *pos + *size >= max_pos {
            break;
        }
        if i % 2 != remainder {
            for i in 0..empty.len() {
                let (pos, size) = empty[i];
                if pos + size >= max_pos {
                    break;
                }
                if x <= size {
                    let to_remove = max_id * (x * (max_pos-x+1) + (x*(x-1))/2);
                    let to_add = max_id * (x * pos + (x*(x-1))/2);
                    results += to_add;
                    results -= to_remove;
                    if x == size {
                        empty.remove(i);
                    } else {
                        empty[i].0 = pos + x;
                        empty[i].1 = size - x;
                    }
                    break;
                }
            }
            max_id -= 1;
        }
        max_pos -= x;
    }

    results
    
}
