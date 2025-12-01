use std::fs::read_to_string;

fn main() {
    let rotations: Vec<String> = read_to_string("rotations.txt").unwrap().lines().map(String::from).collect();
    let mut directions: Vec<bool> = Vec::new(); // Right true
    let mut magnitudes: Vec<i32> = Vec::new();


    for line in &rotations {
        directions.push(line.chars().nth(0).unwrap() == 'R');
        magnitudes.push(line[1..].parse::<i32>().unwrap());
    }

    let mut position: i32 = 50;
    let mut zero_counter: i32 = 0;
    
    // Very inelegant
    // Could be improved by just doing some maths with the resulting position
    // after the full addition/subtraction
    for i in 0..rotations.len() {
        for _ in 0..magnitudes[i].abs() {
            position += if directions[i] { 1 } else { -1 };
            if position == -1 {
                position = 99;
            }
            if position == 100 {
                position = 0;
            }
            if position == 0 {
                zero_counter += 1;
            }
        }
    }

    println!("ZC: {}", zero_counter);
}
