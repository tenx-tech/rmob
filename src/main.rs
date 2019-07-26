use std::process;

fn main() {
    if let Err(e) = rmob::run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}
