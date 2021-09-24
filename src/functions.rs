pub fn run() {
    greeting("Zaryab");
    // Bind function values to variables
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    // Closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C Sum: {}", add_nums(3, 3));
}

fn greeting(name: &str) {
    println!("Hy {}", name);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
