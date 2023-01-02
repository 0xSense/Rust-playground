//Ownership rules 
// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

//integer types

// numeric operations

fn main() {
    //addition
    let sum =  5 + 10;
    //subtraction
    let difference = 9 - 6;
    
    //boolean 
    let t = true;

    let f: bool = false;
    let number = 99;

    if number < 5 {
        println!("Expression no 5")
    } else {
        println!("Expression in not a 5")
    }  
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}