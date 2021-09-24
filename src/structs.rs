// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple Struct
// struct ColorTuple(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Constructor
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_first_name(&mut self, first: &str) {
        self.first_name = first.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    // let mut c = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0,
    // };

    // c.red = 200;

    // println!("Color: {} {} {}", c.red, c.green, c.blue);

    // let mut ct = ColorTuple(255, 0, 0);

    // ct.0 = 200;
    // println!("Color: {} {} {}", ct.0, ct.1, ct.2);

    let mut p = Person::new("Zaryab", "Rafique");

    println!("Person: {}", p.full_name());

    p.set_first_name("Muhammad Zaryab");

    println!("Person: {}", p.full_name());

    println!("Person Tuple: {:?}", p.to_tuple());
}
