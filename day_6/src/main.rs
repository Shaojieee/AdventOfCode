use std::{collections::BTreeSet, f64::consts::{PI, SQRT_2}, time::Instant};

fn main() {
    
    let now = Instant::now();
    let r1 = p1();
    let elapsed = now.elapsed();
    println!("Result for p1: {}", r1);
    println!("part1 : {:.2?}", elapsed);
}

fn p1() -> usize {
    let input_str = include_str!("./input.txt");
    let mut obstacles: BTreeSet<(isize, isize)> = BTreeSet::new();
    let mut position: Option<(isize, isize)> = None;

    let grid: Vec<Vec<char>> = input_str.split('\n').map(|x| x.chars().collect()).filter(|x: &Vec<char>| x.len()!=0).collect();

    let row = isize::try_from(grid.len()).unwrap();
    let col = isize::try_from(grid.get(0).unwrap().len()).unwrap();

    for r in 0..row {
        for c in 0..col {
            let r_usize = usize::try_from(r).expect(&format!("Unable to convert {r}"));
            let c_usize = usize::try_from(c).expect(&format!("Unable to convert {c}"));
            match grid.get(r_usize).expect(&format!("Unable to get row {r_usize}")).get(c_usize).expect(&format!("Unable to get row {r_usize}")) {
                '#' => {
                    obstacles.insert((r,c));
                },
                '^' => {
                    position = Some((r,c));
                },
                _ => {}
            }
        }
    }

    let mut position = position.expect("Unable to find Guard position!");
    let mut direction: (isize, isize, i8) = (-1, 0, 0);
    let mut path: BTreeSet<(isize, isize)> = BTreeSet::from([position]);
    while !(position.0 >= row || position.0 < 0 || position.1 >= col || position.1 < 0) {
        path.insert(position);
        let next_position = (position.0 + direction.0, position.1 + direction.1);

        match obstacles.contains(&next_position) {
            true => {
                direction.2 = (direction.2 + 1) % (4);
                direction.0 = -1*((f64::from(direction.2)*PI/2.0).cos()*SQRT_2).round() as isize;
                direction.1 = ((f64::from(direction.2 - 1)*PI/2.0).cos()*SQRT_2).round() as isize;
            },
            false => {
                position = next_position;
            }
        } 
    }
    path.len()
}

