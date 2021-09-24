pub fn run() {
    let age = 18;
    let check_id = false;

    // If-else
    if age >= 18 && check_id {
        println!("You can vote!");
    } else if age < 18 && check_id {
        println!("You can't vote!");
    } else {
        println!("Give you ID");
    }

    // Shorthand if-else
    let vote_age = if age >= 18 && check_id {
        "You can vote!"
    } else {
        "You can't vote!"
    };

    println!("{}", vote_age);

}