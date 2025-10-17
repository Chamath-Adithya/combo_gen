// ComboGen - Unified Entry Point
// Automatically chooses Ultra-Fast version for performance
// Use --version optimized to select optimized version

mod combo_gen_ultra;
mod combo_gen_optimized;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if user wants optimized version
    let use_optimized = args.contains(&"--version".to_string()) &&
        args.get(args.iter().position(|x| x == "--version").unwrap() + 1) == Some(&"optimized".to_string());

    if use_optimized {
        println!("⚡ Running Optimized version...");
        combo_gen_optimized::main();
    } else {
        println!("🚀 Running Ultra-Fast version (default - best performance)...");
        combo_gen_ultra::main();
    }
}
