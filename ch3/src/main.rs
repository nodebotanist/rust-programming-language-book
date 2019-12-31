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
}
