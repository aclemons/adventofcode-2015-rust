use std::io;
use std::env;

extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;

extern crate pcre;
use pcre::Pcre;

fn mine_advent_coin(input: String, zero_count: i32) -> i32 {
    let mut i = 1;

    let prefix = std::iter::repeat("0").take(zero_count as usize).collect::<String>();
    let pattern = format!("^{}", prefix);
    let mut re = Pcre::compile(&pattern).unwrap();

    let mut md5 = Md5::new();

    loop {
        let next_string = format!("{}{}", input, i.to_string());

        md5.input_str(&next_string);

        let md5sum = md5.result_str();

        md5.reset();

        if re.exec(&md5sum).is_some() {
            break;
        }

        i += 1
    }

    i
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Invalid input");

    let required_zeros = if env::args().count() == 2 &&
                            env::args().nth(1).unwrap() == "--with-six-zeros" {
        6
    } else {
        5
    };

    let lowest_number = mine_advent_coin(input.trim().to_string(), required_zeros);

    println!("The lowest number with {} zeros is {}",
             required_zeros,
             lowest_number);
}
