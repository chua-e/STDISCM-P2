use std::io::BufRead;
use std::{eprintln, io, println};
use std::io::{Write};
use std::fs::File;
use std::path::Path;
use STDISCM_P2::sync::assign;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let filename = "config.txt";
    let (n_instances, start, end) = read_config(filename);

    if n_instances <= 0 {
        eprintln!("Instances need to be greater than or equal to 1.");
        return;
    }

    if start > end {
        eprintln!("Start of the time range must be lesser than the end of the time range.");
        return;
    } else if end > 15 {
        eprintln!("Maximum time range ends at 15.");
        return;
    }

    println!("Enter number of players in party queue by tank, healer, and DPS separated by space (i.e. `10 20 30`)");
    println!();
    print!("Input: ");
    io::stdout().flush().unwrap();

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).expect("Player types needed.");
    let players: Vec<u32> = input.trim().split_whitespace().map(|s| s.parse().expect("Invalid number")).collect();

    if players.len() < 3 {
        eprintln!("There should be at least 3 integers.");
        return
    }

    let t = players[0];
    let h = players[1];
    let d = players [2];

    assign(n_instances, t, h, d, start, end).await;
}

/**
 * Reads the config file in the format where --
 */
fn read_config(filename: &str) -> (u32, u32, u32) {
    let mut n: u32 = 0;
    let mut t1: u32 = 0;
    let mut t2: u32 = 0;

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
                    if let Ok(instances) = second_token.parse::<u32>() {
                        n = instances;
                    }
                } else if first_token == "t1" {
                    if let Ok(bott_time) = second_token.parse::<u32>() {
                        t1 = bott_time;
                    }
                } else if first_token == "t2" {
                    if let Ok(top_time) = second_token.parse::<u32>() {
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