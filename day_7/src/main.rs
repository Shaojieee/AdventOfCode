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

fn p1() -> i64 {
    let input_str = include_str!("./input.txt");
    let mut results = 0;

    input_str
    .split('\n')
    .for_each(|x| {
        let y = x.split_once(':').expect(&format!("Unable to split: {x}"));
        let total: i64 = y.0.trim().parse().unwrap();
        let operands: Vec<i64> = y.1.trim().split(' ').filter(|y| !y.is_empty()).map(|y| y.parse().expect(&format!("Error parsing: {y}"))).collect();
        if p1_recursive(&operands, 0, &total) {
            results += total;
        }
    });

    results
    
}

fn p1_recursive(operands: &[i64], curr: i64, total: &i64) -> bool {
    if curr > *total || operands.len() == 0 {
        return false;
    }
    if operands.len() == 1 {
        if curr + operands[0] == *total || curr * operands[0] == *total {
            return true;
        }
        return false;
    } else {
        if p1_recursive(&operands[1..], curr+operands[0], total) {
            return true;
        }
        if p1_recursive(&operands[1..], curr*operands[0], total) {
            return true;
        }
    }
    false
}

fn p2() -> i64 {
    let input_str = include_str!("./input.txt");
    let mut results = 0;

    input_str
    .split('\n')
    .for_each(|x| {
        let y = x.split_once(':').expect(&format!("Unable to split: {x}"));
        let total: i64 = y.0.trim().parse().unwrap();
        let operands: Vec<i64> = y.1.trim().split(' ').filter(|y| !y.is_empty()).map(|y| y.parse().expect(&format!("Error parsing: {y}"))).collect();
        if p2_recursive(&operands, 0, &total) {
            results += total;
        }
    });

    results
    
}

fn p2_recursive(operands: &[i64], curr: i64, total: &i64) -> bool {
    if curr > *total || operands.len() == 0 {
        return false;
    }
    if operands.len() == 1 {
        if curr + operands[0] == *total || curr * operands[0] == *total {
            return true;
        }
        let concat_total: i64 = format!("{}{}", curr, operands[0]).parse().unwrap();
        if concat_total == *total {
            return true;
        }

        return false;
    } else {
        if p2_recursive(&operands[1..], curr+operands[0], total) {
            return true;
        }
        if p2_recursive(&operands[1..], curr*operands[0], total) {
            return true;
        }

        let new_curr = format!("{}{}", curr, operands[0]).parse().unwrap();
        if p2_recursive(&operands[1..], new_curr, total) {
            return true;
        }
    }
    false
}


