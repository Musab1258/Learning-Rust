// 1
use std::mem::size_of_val;
fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),4); 

    let c2 = '中';
    assert_eq!(size_of_val(&c2),4); 

    println!("Success!");
} 


// 2
fn main() {
    let c1 = '中';
    print_char(c1);
} 

fn print_char(c : char) {
    println!("{}", c);
}


// 3
fn main() {
    let _f: bool = false;

    let t = false;
    if !t {
        println!("Success!");
    }
} 


// 4
fn main() {
    let f = true;
    let t = true || false;
    assert_eq!(t, f);

    println!("Success!");
}


// 5
fn main() {
    let v1: () = ();

    let v = (2, 3);
    assert_eq!(v1, implicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}


// 6
use std::mem::size_of_val;
fn main() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!");
}