use clap::Parser;
use sha2::{Digest, Sha256};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

#[derive(Parser, Debug)]
#[command(version, about = "Finds SHA-256 hashes ending with specified amount of zeros")]
pub struct Args {
    #[arg(short = 'N', long)]
    pub zeros_amount: usize,
    
    #[arg(short = 'F', long)]
    pub results_count: usize,
}

fn validate_args(args: &Args) -> Result<(), String> {
    if args.zeros_amount > 64 {
        return Err("Number of zeros cannot exceed 64".to_string());
    }
    if args.results_count == 0 {
        return Err("Results count must be at least 1".to_string());
    }
    Ok(())
}

fn calculate_hash(number: u64) -> String {
    let mut hasher = Sha256::new();
    hasher.update(number.to_string());
    let hash: [u8; 32] = hasher.finalize().into();
    
    hash.iter()
        .map(|byte| format!("{:02x}", byte))
        .collect()
}

fn has_required_zeros(hash: &str, zeros_amount: usize) -> bool {
    hash.ends_with(&"0".repeat(zeros_amount))
}

fn hash_worker(
    counter: Arc<AtomicU64>,
    hash_sender: mpsc::Sender<(u64, String)>,
    zeros_amount: usize,
) {
    loop {
        let current_number = counter.fetch_add(1, Ordering::Relaxed);
        let hash_hex = calculate_hash(current_number);
        
        if has_required_zeros(&hash_hex, zeros_amount) {
            if hash_sender.send((current_number, hash_hex)).is_err() {
                break;
            }
        }
    }
}

fn spawn_worker_threads(
    zeros_amount: usize,
    counter: Arc<AtomicU64>,
    hash_sender: mpsc::Sender<(u64, String)>,
) {
    let threads_amount = num_cpus::get();
    
    for _ in 0..threads_amount {
        let counter = Arc::clone(&counter);
        let hash_sender = hash_sender.clone();
        let zeros = zeros_amount;
        
        thread::spawn(move || {
            hash_worker(counter, hash_sender, zeros);
        });
    }
}

fn collect_results(
    hash_receiver: mpsc::Receiver<(u64, String)>,
    required_results: usize,
) -> usize {
    let mut results_found = 0;
    
    for (number, hash) in hash_receiver {
        println!("{}, \"{}\"", number, hash);
        results_found += 1;
        
        if results_found >= required_results {
            break;
        }
    }
    
    results_found
}

pub fn run_application(args: Args) -> Result<(), String> {
    validate_args(&args)?;
    
    let threads_amount = num_cpus::get();
    println!("Using {} threads to find {} hashes ending with {} zeros...", 
             threads_amount, args.results_count, args.zeros_amount);

    let counter = Arc::new(AtomicU64::new(1));
    let (hash_sender, hash_receiver) = mpsc::channel();

    spawn_worker_threads(args.zeros_amount, Arc::clone(&counter), hash_sender);

    let results_found = collect_results(hash_receiver, args.results_count);
    println!("Successfully found {} results.", results_found);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_args_valid() {
        let args = Args { zeros_amount: 5, results_count: 10 };
        assert!(validate_args(&args).is_ok());
    }

    #[test]
    fn test_validate_args_too_many_zeros() {
        let args = Args { zeros_amount: 65, results_count: 10 };
        assert!(validate_args(&args).is_err());
    }

    #[test]
    fn test_validate_args_zero_results() {
        let args = Args { zeros_amount: 3, results_count: 0 };
        assert!(validate_args(&args).is_err());
    }

    #[test]
    fn test_calculate_hash() {
        let hash = calculate_hash(1);
        assert_eq!(hash.len(), 64);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_has_required_zeros() {
        assert!(has_required_zeros("abc000", 3));
        assert!(!has_required_zeros("abc001", 3));
        assert!(has_required_zeros("000000", 6));
        assert!(!has_required_zeros("000001", 6));
    }
}
