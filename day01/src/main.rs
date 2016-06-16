use std::io;

fn calculate_floor(input: String) -> (i32, i32) {
    let mut floor = 0;
    let mut basement_position = -1;

    for (index, character) in (0..).zip(input.chars()) {
        if floor == -1 && basement_position == -1 {
            basement_position = index;
        }

        floor += if character == '(' {
            1
        } else if character == ')' {
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
