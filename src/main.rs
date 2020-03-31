fn main() {
    // variables in rust are immutable by default
    let x = 6;
    println!("The value of x is: {}", x);

    // this wont compile
    // x = 5;
    // println!("The value of x is: {}", x);

    let mut y = 5;
    println!("The value of y is: {}", y);

    y = 6;
    println!("The value of y is: {}", y);

    // constants in rust must use `const` and the pattern is to name it
    // in uppercase with underscores for spaces for both names and values
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS: {}", MAX_POINTS);

    // shadowing is when we reassign a variable with a new value, shadowing
    // its name within the scope of the function
    let j = 6;
    let j = j + 1;
    let j = j * 2;
    println!("The value of j is: {}", j);

    // we can also change the type of a variable with shadowing but not mut
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    // this wont compile since the type changes
    // let mut spaces = "   ";
    // spaces = spaces.len()
}
