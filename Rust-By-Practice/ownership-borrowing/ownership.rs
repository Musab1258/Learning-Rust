// 1
fn main() {
    // Use as many approaches as you can to make it work
    let x = String::from("Hello world");
    let y = x.clone();
    println!("{}, {}",x, y);
}


or

fn main() {
    // Use as many approaches as you can to make it work
    let x = "Hello world";
    let y = x;
    println!("{}, {}",x, y);
}

or

fn main() {
    // Use as many approaches as you can to make it work
    let x = &String::from("Hello world");
    let y = x;
    println!("{}, {}",x, y);
}

or

fn main() {
    // Use as many approaches as you can to make it work
    let x = String::from("Hello world");
    let y = x.as_str();
    println!("{}, {}",x, y);
}


// 2
fn main() {
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}


// 3

fn main() {
    let s = give_ownership();
    println!("{}", s);
}

fn give_ownership() -> String {
    let s = String::from("Hello world");
    // Convert String to Vec
    let _s = s.as_bytes();
    s
}


// 4