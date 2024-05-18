// 1
fn main() {
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}


// 2
fn main() {
    print();
 }
 
 // Replace i32 with another type
 fn print() -> () {
    println!("Success!");
 }


// 3
fn main() {
    never_return();
}

fn never_return() -> ! {
    // Implement this function, don't modify the fn signatures
    panic!("This function never returns")
    
}

or

fn main() {
    never_return();
}

use std::thread;
use std::time;

fn never_return() -> ! {
    // implement this function, don't modify fn signatures
    loop {
        println!("I return nothing");
        // sleeping for 1 second to avoid exhausting the cpu resource
        thread::sleep(time::Duration::from_secs(1))
    }
}


// 4
fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };
    
    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}

// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
    panic!()
}

or

// IMPLEMENT this function in THREE ways

use std::thread;
use std::time;

fn never_return_fn() -> ! {
    loop {
        thread::sleep(time::Duration::from_secs(1))
    }
}

or

// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
    unimplemented!()
}

or

// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
    todo!()
}


// 5
fn main() {
    // FILL in the blank
    let b = false;

    let _v = match b {
        true => 1,
        // Diverging functions can also be used in match expression to replace a value of any value
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}