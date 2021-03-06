use std::io;
use std::io::BufRead;
use std::process;

fn main() {
    let first_arg = std::env::args().nth(1).unwrap_or_else(|| String::from("0"));

    if first_arg == "--help" || first_arg == "-h" {
        println!("nth - Select the nth line(s) from STDIN");
        println!();
        println!("Usage:");
        println!("    <your command> | nth <start> [end]");
        println!();
        println!("Options:");
        println!("    start          The start index of lines to print.");
        println!("                   If this is the only argument, only the start'th line will be printed");
        println!();
        println!("    end            The end index to print. Supplying any number less than `start` will have no effect.");
        return;
    }

    let first_arg = first_arg.parse::<usize>();
    match first_arg {
        Ok(start_idx) => {
            let end_idx = std::env::args()
                .nth(2)
                .unwrap_or_else(|| start_idx.to_string())
                .parse::<usize>()
                .unwrap_or(start_idx);

            for (ctr, line) in io::stdin().lock().lines().enumerate() {
                if ctr >= start_idx {
                    match line {
                        Ok(l) => {
                            println!("{}", l);

                            if ctr >= end_idx {
                                process::exit(0);
                            }
                        }
                        Err(e) => {
                            eprintln!("Could not read line: {}", e);
                            process::exit(1);
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Could not parse argument as number: {}", e);
            process::exit(1);
        }
    }
}
