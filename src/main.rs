fn main() {
    /*
    ========= 12/09/2022 Review Questions for next day ==========
    1. Are variables mutable or immutable by default in Rust?
    2. How would you create a mutable variable in rust?
    3. How would you write a constant in Rust?
    4. What is the naming convention for writing constants in Rust?
    5. When declaring a constant, can you use an expression, eg 60 * 60 * 3 on the right side?
    6. When declaring a constant, what is absolutely necessary to do on the left side?
    7. Can you explain shadowing in Rust?
    8. Can you write a simple example of shadowing?
    9. Can you write a slightly more complex example of shadowing?
    */
    // question 1 & 2
    let name_a = "Barry";
    println!("Hello, world!, {name_a}");
    let mut name_b = "Bob";
    println!("Hello {name_b}");
    name_b = "Sarah";
    println!("Hi {name_b}");

    //question 3, 4 and 5
    const HOURS_IN_REGULAR_YEAR: u32 = 365 * 24;
    println!("There are {HOURS_IN_REGULAR_YEAR} hours in a regular year");

    // question 6
    /* Shadowing is when the same variable name, using let is used to store a value of a different type,
    but the variable remains of immutable type. As below for question 8 */
    let minutes_in_reg_year = "525600";
    println!("There are {minutes_in_reg_year} minutes in a regular year and ");
    let minutes_in_reg_year: u32 = minutes_in_reg_year.parse().expect("Parse error!");
    let seconds_in_reg_year = minutes_in_reg_year * 60;
    println!("hence there are {seconds_in_reg_year} seconds in a regular year.");
    // question 7
    let x = 90;
    print!("x is first this value, {x}.");
    let x = -90;
    {
        let x = x * -2;
        println!("\nIn this scope x is {x}.");
    }
    print!("But now x is {x} in the outer scope.");

    /*
    ========= 13/09/2022 Review Questions for next day ==========
    1. Do you always have to write a type annotation?
    2. How can you write a type annotation? Please give a code example?
    3. What is a scalar type in Rust and can you name the 4 types?
    4. What is the difference between signed and unsigned integers?
    5. What and how is _ used in integers? Give an example?
    6. What is a common place you might use ‘usize’ or ‘isize’?
    7. What is integer overflow?
    8. What is a standard floating point number?
    9. Are all floating point numbers signed or unsigned?
    */
}
