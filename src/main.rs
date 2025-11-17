use clap::Parser;
use hash_finder::{Args, run_application};

fn main() {
    let args = Args::parse();
    
    if let Err(e) = run_application(args) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
