pub fn run() {
    let mut name = String::from("Zaryab");

    println!("Name: {}", name);

    // Get the length of the string
    println!("Length: {}", name.len());

    // Push a character to the string
    name.push('R');

    println!("Push: {}", name);
}
