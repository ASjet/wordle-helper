use std::io::{Write};
fn main() {
    let dict_path = "/home/sjet/Documents/wordle_all.txt";
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

    // println!("Secret:");

    // let mut secret = String::new();
    // let secret = if let Ok(_) = std::io::stdin().read_line(&mut secret) {
    //     String::from(secret.trim())
    // } else {
    //     return;
    // };

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
                    if let Some(cand) = wordle_match(&guess, &result, candidate) {
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

fn wordle_match(input: &String, output: &String, candidate: String) -> Option<String> {
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
    let result: String = result.into_iter().collect();
    if result == *output {
        Some(candidate)
    } else {
        None
    }
}
