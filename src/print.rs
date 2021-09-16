pub fn run() {
    println!("Hello from the main.rs file");

    // Basic Formatting
    println!("{} is Expert {}", "Zaryab", "Blockchain Developer");

    // Positional Arguments
    println!(
        "{0} is Expert {1} and {0} is Expert {2} {3}",
        "Zaryab", "Blockchain Developer", "with", "Rust"
    );
}
