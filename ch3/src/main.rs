fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    //let x = 4 (this WILL NOT WORK)
    println!("The value of x is {}", x);

    // characters
    let my_char="C";
    let emoji = "🎂";

    let tup: (bool, i64) = (true, 5_000); // note the _ as a vis indicator in Rust numbers
    let (y, z) = tup;
    println!("The values in y and z are {} and {}", y, z);
    println!("The values in tup are {} and {}", tup.0, tup.1);

    // arrays in rust are fixed size, unlike vectors
    let a = [1, 2, 3, 4, 5];
    for element in a.iter() {
        println!("This array item value is {}", element);
    }
    let b : [i32; 5] = [6, 7, 8, 9, 10];
    // creates an array of 5 with all elements set to 3
    let c = [ 3;5 ];

    another_function(16);
}

// function parameters
fn another_function(x: i32) {
    println!("The value of the parameter is {}", x);
}
