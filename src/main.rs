use clap::Parser;
use sha2::{Digest, Sha256};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

#[derive(Parser, Debug)]
#[command(version, about = "Finds SHA-256 hashes ending with specified amount of zeros")]
struct Args {
    #[arg(short = 'N', long)]
    zeros: usize,
    
    #[arg(short = 'F', long)]
    results: usize,
}

fn main() {
    let args: Args = Args::parse();
    let threads_amount: usize = num_cpus::get();
    
    println!("Using {} threads to find {} hashes ending with {} zeros...", 
             threads_amount, args.results, args.zeros);

    let counter: Arc<AtomicU64> = Arc::new(AtomicU64::new(1));
    let (hash_sender, hash_receiver) = mpsc::channel();

    for _ in 0..threads_amount {
        let counter: Arc<AtomicU64> = Arc::clone(&counter);
        let tx: mpsc::Sender<(u64, String)> = hash_sender.clone();
        let zeros: usize = args.zeros;
        
        thread::spawn(move || {
            loop {
                let current_num: u64 = counter.fetch_add(1, Ordering::Relaxed);
                
                let mut hasher: Sha256 = Sha256::new();
                hasher.update(current_num.to_string());
                let hash: [u8; 32] = hasher.finalize().into();
                
                let hash_hex: String = hash.iter()
                    .map(|byte| format!("{:02x}", byte))
                    .collect();
                
                if hash_hex.ends_with(&"0".repeat(zeros)) {
                    if tx.send((current_num, hash_hex)).is_err() {
                        break;
                    }
                }
            }
        });
    }
    
    drop(hash_sender);
    
    let mut results_found: usize = 0;
    for (number, hash) in hash_receiver {
        println!("{}, \"{}\"", number, hash);
        results_found += 1;
        if results_found >= args.results {
            break;
        }
    }
}
