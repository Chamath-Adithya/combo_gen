// ComboGen - Unified Entry Point
// Automatically chooses Ultra-Fast version for performance
// Use --version optimized to select optimized version
// Use --version fixed to select fixed version

mod combo_gen_ultra;
mod combo_gen_optimized;
mod combo_gen_fixed;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check version selection
    let version = if args.contains(&"--version".to_string()) {
        if let Some(pos) = args.iter().position(|x| x == "--version") {
            if let Some(version_arg) = args.get(pos + 1) {
                version_arg.as_str()
            } else {
                "ultra"
            }
        } else {
            "ultra"
        }
    } else {
        "ultra"
    };

    match version {
        "optimized" => {
            println!("⚡ Running Optimized version...");
            combo_gen_optimized::main();
        }
        "fixed" => {
            println!("🔧 Running Fixed version...");
            combo_gen_fixed::main();
        }
        _ => {
            println!("🚀 Running Ultra-Fast version (default - best performance)...");
            combo_gen_ultra::main();
        }
    }
}
