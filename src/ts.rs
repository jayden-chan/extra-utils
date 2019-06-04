extern crate chrono;

use chrono::prelude::*;
use std::io;
use std::io::BufRead;

fn main() {
    let pattern = std::env::args()
        .nth(1)
        .unwrap_or("[%Y-%m-%d %H:%M:%S]".to_string());

    if pattern == "--help" || pattern == "-h" {
        println!("ts - Append a timestamp to each line in STDIN");
        println!();
        println!("Usage:");
        println!("    <your command> | ts [format]");
        println!();
        println!("Options:");
        println!("    [format]   The format of the timestamp");
        println!("               The default format is [%Y-%m-%d %H:%M:%S]");
    } else {
        for line in io::stdin().lock().lines() {
            match line {
                Ok(l) => {
                    println!("{} {}", Local::now().format(&pattern), l);
                }
                Err(error) => print!("error: {}", error),
            }
        }
    }
}
