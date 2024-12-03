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

// This function does not use the `is_safe` function as converting the report from string to Vec<isize> takes 1 iteration.
// This iteration is not needed, resulting in 1 pass instead of 2 per report.
fn p1() -> usize {
    let input_str = include_str!("./input.txt");

    input_str
    .split('\n')
    .filter(|report| {
        if report.is_empty(){
            return false;
        }
        let mut increase: Option<bool> = None;
        let mut prev_level:Option<isize> = None;
        for x in report.split_whitespace() {
            let curr = x.parse().unwrap();
            match prev_level {
                None => { // first level
                    prev_level = Some(curr);
                },
                Some(prev) => { // after first level
                    if prev == curr || (prev-curr).abs() > 3{
                        return false;
                    }
                    match increase {
                        None => { // second level
                            increase = Some((prev-curr) > 0);
                        },
                        Some(y) => { // check if consistently increasing or decreasing
                            if (y && (prev-curr) < 0) || (!y && (prev-curr) > 0){
                                return false;
                            }
                        }
                    }
                    prev_level = Some(curr);
                }
            }
        };
        return true;
    }).count()
}


fn p2() -> usize {
    let input_str = include_str!("./input.txt");

    input_str
    .split("\n")
    .filter(|report| {
        let report = report.split_whitespace().map(|x| x.parse().unwrap()).collect();
        if is_safe(&report) {
            return true;
        }
        for i in 0..report.len() { // Manually loop through to see which to remove
            let mut dampened_report = report.clone();
            dampened_report.remove(i);
            if is_safe(&dampened_report){
                return true;
            }
        }
        false
    })
    .count()
}

// Function to check if report is safe
fn is_safe(report: &Vec<isize>) -> bool {
    if report.is_empty(){
        return false;
    }
    let mut increase: Option<bool> = None;
    let mut prev_level:Option<isize> = None;
    for curr in report {
        let curr = *curr;
        match prev_level {
            None => { // first level
                prev_level = Some(curr);
            },
            Some(prev) => { // after first level
                if prev == curr || (prev-curr).abs() > 3{
                    return false;
                }
                match increase {
                    None => { // second level
                        increase = Some((prev-curr) > 0);
                    },
                    Some(y) => { // check if consistently increasing or decreasing
                        if (y && (prev-curr) < 0) || (!y && (prev-curr) > 0){
                            return false;
                        }
                    }
                }
                prev_level = Some(curr);
            }
        }
    }
    true
}
