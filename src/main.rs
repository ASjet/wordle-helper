use rand::Rng;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
fn main() {
    let dict_path = "wordle_dict.txt";
    let argc: usize = std::env::args().len();
    let args: Vec<String> = std::env::args().collect();

    let mut candidates: Vec<String> = Vec::new();
    if let Ok(file) = std::fs::read(dict_path) {
        if let Ok(content) = String::from_utf8(file) {
            let lines: Vec<&str> = content.split("\n").collect();
            for line in lines {
                candidates.push(String::from(line));
            }
        }
    }

    println!("Loaded {} candidate words", candidates.len());

    if argc == 2 && args[1] == "-p" {
        wordle_play(candidates);
    } else {
        wordle_helper(candidates);
    }
}

fn wordle_match(input: &String, candidate: &String) -> String {
    let mut cnt: std::collections::HashMap<char, i32> = std::collections::HashMap::new();

    let mut result = vec!['.'; 5];
    for (i, (w, c)) in std::iter::zip(input.chars(), candidate.chars()).enumerate() {
        let count = cnt.entry(w).or_insert(0);
        if w == c {
            result[i] = '1';
            *count += 1;
        } else {
            let mut no = *count + 1;
            for (j, k) in candidate.chars().enumerate() {
                if k == w {
                    no = if i != j && no > 0 { no - 1 } else { no };
                }
            }
            result[i] = if no > 0 { '0' } else { '.' };
            *count += 1;
        }
    }
    return result.into_iter().collect();
}

fn wordle_if_match(input: &String, output: &String, candidate: String) -> Option<String> {
    let result = wordle_match(input, &candidate);
    return if result == *output {
        Some(candidate)
    } else {
        None
    };
}

fn wordle_helper(mut candidates: Vec<String>) {
    for i in 1..7 {
        let mut next_candidates: Vec<String> = Vec::new();

        print!("{}th guess:", i);
        std::io::stdout().flush().unwrap();
        let mut guess = String::new();
        let guess = if let Ok(_) = std::io::stdin().read_line(&mut guess) {
            String::from(guess.trim())
        } else {
            return;
        };
        assert_eq!(5, guess.len());

        print!("{}th result:", i);
        std::io::stdout().flush().unwrap();
        let mut result = String::new();
        let result = if let Ok(_) = std::io::stdin().read_line(&mut result) {
            String::from(result.trim())
        } else {
            return;
        };
        assert_eq!(5, result.len());

        loop {
            if let Some(candidate) = candidates.pop() {
                if candidate.len() == 5 {
                    if let Some(cand) = wordle_if_match(&guess, &result, candidate) {
                        next_candidates.push(cand);
                    }
                }
            } else {
                break;
            }
        }

        let rest = next_candidates.len();
        if rest == 0 {
            println!("Oops! There is no matching word!");
            return;
        }
        println!("{} matched.", rest);
        if rest < 10 {
            println!("{:?}", next_candidates);
            if rest == 1 {
                println!("Congratulations!");
                return;
            }
        } else {
            print!("Show them?[y/N]");
            std::io::stdout().flush().unwrap();
            let mut show = String::new();
            let show = if let Ok(_) = std::io::stdin().read_line(&mut show) {
                show.as_bytes()[0].to_ascii_lowercase() as char
            } else {
                return;
            };
            if show == 'y' {
                println!("{:?}", next_candidates);
            }
        }
        candidates = next_candidates;
    }
}

fn wordle_play(candidate: Vec<String>) -> bool {
    let length = candidate.len();
    let sn: usize = rand::thread_rng().gen_range(0..length);
    let secret: &String = &candidate[sn];
    let gotcha = String::from("11111");

    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    let mut green = ColorSpec::new();
    let green = green.set_fg(Some(Color::White));
    let green = green.set_bg(Some(Color::Green));

    let mut gray = ColorSpec::new();
    let gray = gray.set_fg(Some(Color::White));
    let gray = gray.set_bg(Some(Color::Rgb(76, 74, 72)));

    let mut yellow = ColorSpec::new();
    let yellow = yellow.set_fg(Some(Color::White));
    let yellow = yellow.set_bg(Some(Color::Yellow));

    for i in 1..7 {
        print!("{}th guess: ", i);
        std::io::stdout().flush().unwrap();
        let mut guess = String::new();
        let guess = if let Ok(_) = std::io::stdin().read_line(&mut guess) {
            String::from(guess.trim())
        } else {
            eprint!("Failed to read input");
            return false;
        };
        let result = wordle_match(&guess, &secret);
        for (c, r) in std::iter::zip(guess.chars(), result.chars()) {
            match r {
                '1' => {
                    if let Ok(_) = stdout.set_color(green) {};
                    print!("{}", c);
                }
                '0' => {
                    if let Ok(_) = stdout.set_color(gray) {};
                    print!("{}", c);
                }
                _ => {
                    if let Ok(_) = stdout.set_color(yellow) {};
                    print!("{}", c);
                }
            }
            std::io::stdout().flush().unwrap();
        }
        if let Ok(_) = stdout.reset() {};
        println!();
        if result == gotcha {
            println!("Congratulations!");
            return true;
        }
    }
    println!("You lost![{}]", secret);
    return false;
}
