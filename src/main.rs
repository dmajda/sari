use std::{env, process};

fn main() {
    let args = env::args();

    if args.len() == 1 {
        println!("Usage: sari <expr>...");
        process::exit(1);
    }

    let exprs = args.skip(1);

    for expr in exprs {
        match sari::eval(&expr) {
            Ok(value) => println!("{value}"),
            Err(e) => {
                eprintln!("{e}");
                process::exit(1);
            }
        }
    }
}
