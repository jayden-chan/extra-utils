use std::io;
use std::io::BufRead;

fn main() {
    let first_arg = std::env::args().nth(1).unwrap_or_else(|| String::from("0"));

    if first_arg == "--help" || first_arg == "-h" {
        println!("nr - Prepend line numbers to STDIN");
        println!();
        println!("Usage:");
        println!("    <your command> | nr [minwidth] [offset]");
        println!();
        println!("Options:");
        println!("    [minwidth]    The minimum width of number column");
        println!("                  Select 0 for no minimum width");
        println!();
        println!("    [offset]      Starting line index. Default is 1");
        return;
    }

    let width = first_arg.parse::<usize>().unwrap_or(0);
    let mut ctr = std::env::args()
        .nth(2)
        .unwrap_or_else(|| String::from("1"))
        .parse::<i32>()
        .unwrap_or(1);

    for line in io::stdin().lock().lines() {
        match line {
            Ok(l) => {
                println!("{ctr:<width$} {line}", ctr = ctr, line = l, width = width);
            }
            Err(error) => print!("error: {}", error),
        }

        ctr += 1;
    }
}
