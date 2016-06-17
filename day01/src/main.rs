use std::io;

fn calculate_floor(input: String) -> (i32, u32) {
    let mut floor = 0;
    let mut basement_position: Option<u32> = None;

    for (index, character) in (0..).zip(input.chars()) {
        if floor == -1 && basement_position.is_none() {
            basement_position = Some(index);
        }

        floor += match character {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
    }

    (floor, basement_position.unwrap())
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
