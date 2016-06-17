use std::io;
use std::io::BufRead;
use std::env;

extern crate pcre;
use pcre::Pcre;

fn nice(line: String) -> bool {
    let mut vowel_re = Pcre::compile("[aeiou].*[aeiou].*[aeiou]").unwrap();
    let mut repeat_re = Pcre::compile("(.)\\1{1,}").unwrap();
    let mut illegal_seq_re = Pcre::compile("ab|cd|pq|xy").unwrap();

    vowel_re.exec(&line).is_some() && repeat_re.exec(&line).is_some() &&
    illegal_seq_re.exec(&line).is_none()
}

fn nice_v2(line: String) -> bool {
    let mut repeat_re1 = Pcre::compile("(.).\\1{1}").unwrap();
    let mut repeat_re2 = Pcre::compile("(..).*\\1{1}").unwrap();

    repeat_re1.exec(&line).is_some() && repeat_re2.exec(&line).is_some()
}

fn count_nice_strings(input: io::Lines<io::StdinLock>, use_new_rules: bool) -> usize {
    input.into_iter().map(|x| x.unwrap()).fold(0, |sum, line| {
        sum +
        match use_new_rules {
            true => {
                if nice_v2(line) {
                    1
                } else {
                    0
                }
            }
            _ => {
                if nice(line) {
                    1
                } else {
                    0
                }
            }
        }
    })
}

fn main() {
    let stdin = io::stdin();

    let use_new_rules = env::args().count() == 2 &&
                        env::args().nth(1).unwrap() == "--with-new-rules";

    let nice_string_count = count_nice_strings(stdin.lock().lines(), use_new_rules);

    println!("Number of nice strings{} {}",
             if use_new_rules {
                 " using new rules"
             } else {
                 ""
             },
             nice_string_count);
}
