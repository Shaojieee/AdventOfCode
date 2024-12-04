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
    // Keep track of ans
    let mut total: usize = 0;
    // Store letters match for 'mul'
    let mut mul: usize = 0;
    // Store numbers to multiply
    let mut nums: Vec<usize> = vec![0;2]; 
    let mut instructions = input_str
    .split("")
    .into_iter();

    while let Some(l) = instructions.next() {
        match mul {
            0 => {
                if l == "m" {
                    mul += 1;
                }
            },
            1 => {
                if l == "u" {
                    mul += 1;
                }else {
                    mul = 0;
                }
            },
            2 => {
                if l == "l" {
                    mul += 1;
                } else {
                    mul = 0
                }
            },
            3 => {
                if l == "("{
                    let mut num_i = 0; // index for which operand to update
                    while let Some(ns) = instructions.next() {
                        let n: Result<usize, _> = ns.parse();
                        if let Ok(n) = n { // valid number
                            nums[num_i] = nums[num_i] * 10 + n;
                        } else if ns == "," && num_i == 0 { // Seperator for operand
                            num_i += 1
                        } else if ns == ")" && num_i == 1 { // End of current mul instruction
                            total += nums[0] * nums[1];
                            break;
                        } else { // Not valid character
                            break;
                        }
                    }
                }
                mul = 0;
                nums[0] = 0;
                nums[1] = 0;
            
            },
            _ => panic!("Will not come here!")
        }
    }
    total
}


fn p2() -> usize {
    let input_str = include_str!("./input.txt");
    // Keep track of ans
    let mut total: usize = 0;
    // Store letters match for 'mul'
    let mut mul: usize = 0;
    // Store numbers to multiply
    let mut nums: Vec<usize> = vec![0;2]; 
    // Keep track of enabled or not
    let mut enabled: bool = true;
    // Store letters match for 'do'
    let mut toggle: usize = 0;
    let mut instructions = input_str
    .split("")
    .into_iter();

    while let Some(l) = instructions.next() {
        match l {
            "m" => {
                toggle = 0;
                mul = 1;
            },
            "u" => {
                toggle = 0;
                if mul == 1 {
                    mul = 2;
                } else {
                    mul = 0;
                }
            },
            "l" => {
                toggle = 0;
                if mul == 2 {
                    mul = 3;
                } else {
                    mul = 0
                }
            },
            "(" => {
                // For mul instructions
                if mul == 3 {
                    if !enabled { continue; }
                    let mut num_i = 0; // index for which operand to update
                    while let Some(ns) = instructions.next() {
                        let n: Result<usize, _> = ns.parse();
                        if let Ok(n) = n { // valid number
                            nums[num_i] = nums[num_i] * 10 + n;
                        } else if ns == "," && num_i == 0 { // Seperator for operand
                            num_i += 1
                        } else if ns == ")" && num_i == 1 { // End of current mul instruction
                            total += nums[0] * nums[1];
                            break;
                        } else if ns == "d" { // Not valid character
                            toggle = 1;
                            break;
                        } else if ns == "m" {
                            mul = 1;
                            break;
                        } else {
                            mul = 0;
                            toggle = 0;
                            break
                        }
                    }
                }
                nums[0] = 0;
                nums[1] = 0;

                // For enabling / disabling
                if toggle == 2 || toggle == 5 {
                    let to_enable: bool = toggle == 2;
                    if let Some(ns) = instructions.next() {
                        if ns ==")" {
                            enabled = to_enable;
                            toggle = 0;
                            mul = 0;
                        } else if ns == "m" {
                            mul = 1;
                            toggle = 0;
                        } else if ns == "d" {
                            toggle = 1;
                            mul = 0;
                        } else {
                            mul = 0;
                            toggle = 0;
                        }
                    }
                }
            },
            "d" => {
                mul = 0;
                toggle = 1;
            },
            "o" => {
                mul = 0;
                if toggle == 1 {
                    toggle =2;
                } else {
                    toggle = 0;
                }
            },
            "n" => {
                mul = 0;
                if toggle == 2 {
                    toggle =3;
                } else {
                    toggle = 0;
                }
            },
            "'" => {
                mul = 0;
                if toggle == 3 {
                    toggle = 4;
                } else {
                    toggle = 0;
                }
            },
            "t" => {
                mul = 0;
                if toggle == 4 {
                    toggle = 5;
                } else {
                    toggle = 0;
                }
            },
            _ => {
                mul = 0;
                toggle = 0;
            }
        }
    }
    total
}
