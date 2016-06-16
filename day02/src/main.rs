use std::io;
use std::io::BufRead;

fn calculate_amount(input: io::Lines<io::StdinLock>) -> (i32, i32) {
    let mut paper = 0;
    let mut ribbon = 0;

    for line in input {
        let mut dimensions =
            line.unwrap().split("x").map(|arg| arg.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let l = dimensions[0];
        let w = dimensions[1];
        let h = dimensions[2];

        dimensions.sort();

        let total_area = ((l * w) + (w * h) + (h * l)) * 2;
        let slack = dimensions.iter().take(2).fold(1, |x, y| x * y);

        let length_of_present_ribbon = dimensions.iter().take(2).fold(0, |x, y| x + y) * 2;
        let length_of_bow = dimensions.iter().fold(1, |x, y| x * y);

        paper += total_area + slack;
        ribbon += length_of_present_ribbon + length_of_bow;
    }

    (paper, ribbon)
}

fn main() {
    let stdin = io::stdin();

    let (paper, ribbon) = calculate_amount(stdin.lock().lines());

    println!("The elves should order {} sq ft of paper and {} ft of ribbon",
             paper,
             ribbon);
}
