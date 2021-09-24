pub fn run() {
    let mut name = String::from("Zaryab");

    println!("Name: {}", name);

    // Push a character to the string
    name.push(' ');

    // Push a string to the string
    name.push_str("Rafique");

    // Capacity in bytes
    println!("Capacity: {}", name.capacity());

    // Get the length of the string
    println!("Length: {}", name.len());

    // Check if string is empty
    println!("Is empty: {}", name.is_empty());

    // Contains
    println!("Contains 'Z': {}", name.contains("Z"));

    // Replace
    println!("Replace: {}", name.replace("Z", "A"));

    // Loop through string by whitespace
    for word in name.split_whitespace() {
        println!("Word: {}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("Push: {}", name);
}
