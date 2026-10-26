use std::io::BufRead;
use std::{io, println};
use std::io::{Write};
use std::fs::File;
use std::path::Path;

fn main() {
    let filename = "config.txt";
    let (n_instances, fastest_time, slowest_time) = read_config(filename);

    println!("Enter number of players in party queue by tank, healer, and DPS separated by space (i.e. `10 20 30`)");
    println!();
    print!("Input: ");
    io::stdout().flush().unwrap();

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).expect("Player types needed.");
    let players: Vec<i32> = input.trim().split_whitespace().map(|s| s.parse().expect("Invalid number")).collect();

    if players.len() < 3 {
        eprintln!("There should be at least 3 integers.");
    }

    let t = players[0];
    let h = players[1];
    let d = players [2];

    
}

/**
 * Reads the config file in the format where --
 */
fn read_config(filename: &str) -> (u64, u64, u64) {
    let mut n: u64 = 0;
    let mut t1: u64 = 0;
    let mut t2: u64 = 0;

    let path = Path::new(filename);
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return (0, 0, 0);
        }
    };

    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(text) => {
                let mut tokens = text.split_whitespace();
                let first_token = tokens.next().unwrap_or("");
                let second_token = tokens.next().unwrap_or("");

                if first_token == "n" {
                    if let Ok(instances) = second_token.parse::<u64>() {
                        n = instances;
                    }
                } else if first_token == "t1" {
                    if let Ok(bott_time) = second_token.parse::<u64>() {
                        t1 = bott_time;
                    }
                } else if first_token == "t2" {
                    if let Ok(top_time) = second_token.parse::<u64>() {
                        t2 = top_time;
                    }
                }
            },
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    println!("Config file: ");
    println!("Number of instances: {}", n);
    println!("Fastest clear time: {}", t1);
    println!("Slowest clear time: {}", t2);
    println!();

    return (n, t1, t2);
}