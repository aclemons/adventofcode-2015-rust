use std::io;

fn calculate_floor(input: String) -> (i32, i32) {
    let mut floor = 0;
    let mut basement_position = -1;

    for (i, c) in input.chars().enumerate() {
        if floor == -1 && basement_position == -1 {
            basement_position = i as i32;
        }

        floor += if c == '(' {
            1
        } else if c == ')' {
            -1
        } else {
            0
        };
    }

    (floor, basement_position)
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Invalid input");

    let (floor, basement_position) = calculate_floor(input);

    println!("Santa should be on floor: {} and first entered the basement at {}",
             floor,
             basement_position);
}
