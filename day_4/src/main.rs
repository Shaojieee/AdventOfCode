use std::{collections::HashMap, f64::consts::PI, time::Instant};

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
    let mut results = 0;

    let letters: Vec<Vec<char>> = input_str.split('\n')
    .map(|x| x.chars().collect())
    .collect();

    let row = letters.len();
    let col = letters[0].len();

    // position from start of string and the character that must be at that position
    let lookup: HashMap<usize, char> = HashMap::from([
        (1, 'M'), (2, 'A'), (3, 'S')
    ]);

    for r in 0..row {
        for c in 0..col {
            if letters[r][c] != 'X' { continue; }
            
            for d in 0..8{ // iterating through all directions, starting from north in clockwise direction
                // calculating the offsets for a direction
                let angle = (PI / 4.0) * f64::from(d);
                let v_angle = angle.cos();
                // cannot check v_angle == 0 as f64 computations are not exact
                let v_direction = if v_angle < -0.7 { -1 } else if v_angle > 0.7 { 1 } else { 0 };
                let h_angle = (angle-(PI/2.0)).cos();
                let h_direction = if h_angle < -0.7 { -1 } else if h_angle > 0.7 { 1 } else { 0 };
                let mut valid = true;
                for i in 1..4 { // checking for MAS in the specific direction
                    let r: isize = r.try_into().unwrap();
                    let c: isize = c.try_into().unwrap();
                    let v_offset:isize = v_direction * i + r;
                    let h_offset:isize = h_direction * i + c;

                    // ensuring the indexes are valid
                    if v_offset < 0 || v_offset >= row.try_into().unwrap() || h_offset < 0 || h_offset >= col.try_into().unwrap() {
                        valid = false;
                        break;
                    }

                    let v_offset:usize = v_offset.try_into().unwrap();
                    let h_offset:usize = h_offset.try_into().unwrap();
                    if letters[v_offset][h_offset] != *lookup.get(&i.try_into().unwrap()).unwrap() {
                        valid = false; 
                        break;
                    }  
                }
                if valid {
                    results += 1;
                }
            }
        }
    }
    results 
}


fn p2() -> usize {
    let input_str = include_str!("./input.txt");
    let mut results = 0;

    let letters: Vec<Vec<char>> = input_str.split('\n')
    .map(|x| x.chars().collect())
    .collect();

    let row = letters.len();
    let col = letters[0].len();

    for r in 0..row {
        for c in 0..col {
            if letters[r][c] != 'A' { continue; }
            
            let mut valid = true;
            for d in [1,3]{ // for each diagonals
                let mut v_offsets: Vec<usize> = vec![0,0];
                let mut h_offsets: Vec<usize> = vec![0,0];
                for i in 0..=1{ // getting the coordinates for both ends of the diagonals
                    let i: i32 = i.try_into().unwrap();
                    let angle = (PI / 4.0) * f64::from(d + i * 4);
                    let v_angle = angle.cos();
                    let v_direction = if v_angle < -0.7 { -1 } else if v_angle > 0.7 { 1 } else { 0 };
                    let h_angle  = (angle-(PI/2.0)).cos();
                    let h_direction = if h_angle < -0.7 { -1 } else if h_angle > 0.7 { 1 } else { 0 };

                    let r: isize = r.try_into().unwrap();
                    let c: isize = c.try_into().unwrap();
                    let v_offset:isize = v_direction + r;
                    let h_offset:isize = h_direction + c;

                    if v_offset < 0 || v_offset >= row.try_into().unwrap() || h_offset < 0 || h_offset >= col.try_into().unwrap() {
                        valid = false;
                        break;
                    }

                    let i: usize = i.try_into().unwrap();

                    v_offsets[i] = v_offset.try_into().unwrap();
                    h_offsets[i] = h_offset.try_into().unwrap();
                }
                
                if !valid { break; } // Not valid indexes

                // checking for either MAS or SAM
                if valid && !(
                    (letters[v_offsets[0]][h_offsets[0]] == 'M' && letters[v_offsets[1]][h_offsets[1]] == 'S') 
                    || 
                    (letters[v_offsets[0]][h_offsets[0]] == 'S' && letters[v_offsets[1]][h_offsets[1]] == 'M')
                ) { 
                    valid = false; 
                    break;
                }  
            }
            if valid {
                results += 1;
            }
        }
    }
    results 
}
