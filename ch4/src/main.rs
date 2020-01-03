fn main() {
    let x = 5;
    let y = x;

    println!("y is {} and x is {}", x, y);

    let s1 = String::from("Hello");
    let s2 = s1; // s1 is now invalid
    let s3 = s2.clone();

    // println!("{}", s1); won't work
    println!("s2 is {} and s3 is {}", s2, s3);

    let s = String::from("World");
    take_ownership(s); // s is now invalid in this scope, but the data remains on the heap
    // println!("{}", s); would error

    let z = 5;
    make_copy(z);
    println!("z is: {}", z); // this still works because z was copied, not moved

    let s4 = gives_ownership();
    let s5 = String::from("Foo");
    let s6 = takes_and_gives_back(s5);
    println!("s5 is invalid, s4 is {}, and s6 is {}", s4, s6);
}

fn take_ownership(some_string: String){
    println!("Some string is: {}", some_string);
}// some_string goes out of scope, and the memory is cleaned up

fn make_copy(some_number: i32){
    println!("Some number is: {}", some_number);
}

fn gives_ownership () -> String {
    let result = String::from("Bar");
    result
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}