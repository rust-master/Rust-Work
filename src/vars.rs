pub fn run() {
    let name = "Zaryab";
    let mut age = 22;
    println!("My Name is , {}! and I am {}", name, age);
    age = 23;
    println!("My Name is , {}! and I am {}", name, age);

    // Define const
    const ID: i32 = 103;
    println!("ID is {}", ID);

    // Assign multiple variables
    let (name, age) = ("Zaryab", 22);
    println!("My Name is , {}! and I am {}", name, age);
}
