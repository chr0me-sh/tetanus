extern crate tetanus;

use std::process::exit;

fn run() -> Result<(), tetanus::Error> {
    Ok(())
}

fn main() {
    let result = run();

    match result {
        Ok(_)  => exit(0),
        Err(e) => {
            eprintln!("{}", e);
            exit(1)
        }
    }
}