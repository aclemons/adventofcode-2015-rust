use std::io;
use std::env;

fn calculate_move(x: i32, y: i32, direction: char) -> (i32, i32) {
    let mut new_coordinates = (x, y);

    match direction {
        '<' => new_coordinates.0 -= 1,
        '>' => new_coordinates.0 += 1,
        '^' => new_coordinates.1 -= 1,
        'v' => new_coordinates.1 += 1,
        _ => {}
    }

    new_coordinates
}

fn number_of_houses(input: String, robo_santa: bool) -> i32 {
    let mut santa_coordinates = vec![(0, 0)];
    let mut robo_santa_coordinates = vec![(0, 0)];

    for (index, direction) in (0..).zip(input.chars()) {
        let target = if robo_santa && index % 2 == 0 {
            &mut robo_santa_coordinates
        } else {
            &mut santa_coordinates
        };

        let &(x, y) = target.last().unwrap();

        target.push(calculate_move(x, y, direction));
    }

    let mut coordinates =
        santa_coordinates.iter().chain(robo_santa_coordinates.iter()).collect::<Vec<_>>();

    coordinates.sort_by(|a, b| a.cmp(b));
    coordinates.dedup();

    coordinates.len() as i32
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Invalid input");

    let use_robo_santa: bool = if env::args().count() == 2 &&
                                  env::args().nth(1).unwrap() == "--with-robo-santa" {
        true
    } else {
        false
    };

    let count = number_of_houses(input, use_robo_santa);

    println!("Only {} houses received presents", count);
}

