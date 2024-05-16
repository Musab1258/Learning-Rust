// 1
fn main() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}


// 2
fn main() {
    let mut x = 1;
    x += 2; 
    
    assert_eq!(x, 3);
    println!("Success!");
}


// 3
fn main() {
    let x: i32 = 10;
    let y: i32 = 15;
    {
        let y: i32 = 5;
        println!("Inner scope value of x is {} and value of y is {}", x, y);
    }
    println!("Outer scope value of x is {} and value of y is {}", x, y); 
}


// 4
fn main() {
    let x = define_x();
    println!("{}, world", x); 
}

fn define_x() -> String {
    let x = "hello".to_string();
    x
}

// or

fn main() {
    let x = define_x();
    println!("{}, world", x); 
}

fn define_x() -> &'static str {
    let x = "hello";
    x
}

// 5
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".
}


// 6
fn main() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    let x = x; 


    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!"; 

    println!("Success!");
}


// 7
fn main() {
    let _x = 1;
}

// or

#[allow(unused_variables)]
fn main() {
    let x = 1;
}


// 8

// Fix the error below with least amount of modification
fn main() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}


// 9

fn main() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [3, 2]);

    println!("Success!");
} 