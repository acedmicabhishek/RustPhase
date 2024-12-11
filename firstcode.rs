use std::collections::HashMap;
use std::thread;

fn main() {
    // Your personal information
    let name = "Abhis";
    let age = 23;
    let height_cm = 174;

    println!("Personal Information:");
    println!("Name: {}", name);
    println!("Age: {} years", age);
    println!("Height: {} cm", height_cm);

    // Variable declarations and assignments
    let x = 5;
    let mut y = 10;
    y = 20;

    let x: i32 = 42; // signed int for 32bit
    let y = 3.14; // f64 by default
    let is_active = true;
    let chars = ('a', 42, 3.14);
    let arr = [1, 2, 3, 4, 5];

    // Function call to greet
    println!("{}", greet(name));

    // Conditional check
    if x > 0 {
        println!("Positive number");
    } else {
        println!("Non-positive number");
    }

    // Loop example
    let mut count = 0;
    loop {
        if count == 5 { 
            break;
         }
        println!("{}", count);
        count += 1;
    }

    // For loop example
    for i in 0..5 {
        println!("{}", i);
    }

    // Borrowing and references example
    let s1 = String::from("Hello");
    let s2 = &s1;  // immutable borrow
    let s3 = &s1;  // valid

    let mut s4 = String::from("World");
    let s5 = &mut s4;  // mutable borrow
    // let s6 = &mut s4;  // error, cannot borrow mutably twice

    // Longest string function
    let result = longest(s2, s3);
    println!("Longest string: {}", result);

    // Struct example: Person
    let person = Person { name: String::from("Abhis"), age: 23 };
    println!("Person's Name: {}, Age: {}", person.name, person.age);

    // Enum example: Direction
    let move_direction = Direction::Up;
    match move_direction {
        Direction::Up => println!("Moving Up"),
        Direction::Down => println!("Moving Down"),
        Direction::Left => println!("Moving Left"),
        Direction::Right => println!("Moving Right"),
    }

    // Thread example
    let handle = thread::spawn(|| {
        println!("Hello from a thread!");
    });
    handle.join().unwrap();

    // Vector example
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("Vector: {:?}", vec);

    // HashMap example
    let mut map = HashMap::new();
    map.insert("name", "Abhis");
    println!("HashMap: {:?}", map);

    // Trait and struct example: Speak
    let dog = Dog;
    let cat = Cat;
    dog.speak();
    cat.speak();

    // Macro example
    say_hello!();
}

// Greet function
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// Longest string function
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

// Struct definition: Person
struct Person {
    name: String,
    age: u32,
}

// Enum definition: Direction
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Trait definition: Speak
trait Speak {
    fn speak(&self);
}

// Implementing Speak trait for Dog
struct Dog;
impl Speak for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

// Implementing Speak trait for Cat
struct Cat;
impl Speak for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

// Macro definition
macro_rules! say_hello {
    () => {
        println!("Hello, world!");
    };
}
