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

or


fn main() {
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("Hello world");
    s
}


// 4
fn main() {
    let s = String::from("Hello World");

    print_str(s.clone());

    println!("{}", s);
}

fn print_str(s: String)  {
    println!("{}",s);
}

or

fn main() {
    let s = String::from("Hello World");

    print_str(&s);

    println!("{}", s);
}

fn print_str(s: &String)  {
    println!("{}",s);
}


// 5
fn main() {
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}


// 6
fn main() {
    let s = String::from("Hello ");
    
    let mut s1 = s;

    s1.push_str("World!");

    println!("Success!");
}


// 7
fn main() {
    let x = Box::new(5);
    
    let mut y = Box::new(2);      // update this line, don't change other lines!
    
    *y = 4;
    
    assert_eq!(*x, 5);

    println!("Success!");
}


// 8
fn main() {
    let t = (String::from("hello"), String::from("world"));
 
    let _s = t.0;
 
    // Modify this line only, don't use `_s`
    println!("{:?}", t.1);
 }


// 9 

fn main() {
    let t = (String::from("hello"), String::from("world"));
 
     // Fill the blanks
     let (s1, s2) = t.clone();
 
     println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}

or

fn main() {
    let t = (String::from("hello"), String::from("world"));
 
     // Fill the blanks
     let (s1, s2) = (&t.0, &t.1);
 
     println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}