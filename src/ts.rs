extern crate chrono;

use chrono::prelude::*;
use std::io;
use std::io::BufRead;

fn main() {
    let pattern = std::env::args()
        .nth(1)
        .unwrap_or("%Y-%m-%d %H:%M:%S".to_string());

    for line in io::stdin().lock().lines() {
        match line {
            Ok(l) => {
                println!("[{}] {}", Local::now().format(&pattern), l);
            }
            Err(error) => print!("error: {}", error),
        }
    }
}
