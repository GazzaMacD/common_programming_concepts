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

    /* == Answers 14/09 == */
    //1. No, some type inference is done by Rust but all types must be known at compile time
    //2 & 3.
    let x: u32 = 78_954; //integer
    let y: f64 = 456.45; //float
    let c: char = 'c'; //char *note the single quotes
    let t: bool = true; // boolean
    print!(
        "These are the four scalar types in Rust.
    1. integer --> {x}
    2. float   --> {y}
    3. char    --> {c}
    4. boolean --> {t}\n"
    );
    //4. Signed integers are integers that accept a negative number where as unsigned are 0 or positive.
    let _signed: i16 = -4;
    let _unsigned: u16 = 7;
    //5. _ is used as a convenience space separator for readability --> see x: u32 above.
    //6. usize or isize are often used for indexing a collection
    //7. integer overflow is when by some programming calculation the integer exceeds it's bit size, it is dealt with
    //   in various ways but most throw an error and should be dealt with gracefully, there are methods available for
    //   that purpose.
    //8 & 9. See above for standard floating point, f64 and they are always signed.

    /*
    ========= 14/09/2022 Review Questions for next day ==========
    1.  The char type is specified how?
    2.  Are they unicode? Are emoji, kanji etc available?
    3.  Other that scalar types what are the other primitive types in Rust?
    4.  Which type is fixed in length and cannot grow or shrink? Can you write an example?
    5.  Is destructuring available to this type? Can you write an example?
    6.  In python we access a value in this type using x[0] syntax, write example for Rust.
    7.  What type represents and empty tuple, ()?
    8.  Arrays and tuples differ in what ways in Rust?
    9.  Are Arrays fixed or flexible in length?
    10. If you want a flexible array, what type would you use?
    11. Are arrays allocated to the stack or the heap?
    12. How can you explicitly write the type of an array?
    13. If you want an array of same values how could you write that? Give an example.
    14. How can you access the second value in an array, please write the code.
    15. If you try to access an array value out of index, what happens if at runtime?
    16. The above behaviour is the same as C? Why is this a good thing?
    */

    /* == Answers 15/09 == */
    // 1 & 2. Char types are specified with single quotes as below and are unicode.
    let _b: char = '4';

    //3. The other primitive types are compount types.

    //4. Tuples are of fixed length and cannot grow or shrink. Type annotations are optional.
    let _t = (1, 'b', "trust");
    let tup: (u8, char, bool) = (1, 't', true);

    //5. Yes destructuring is available
    let (_k, _d, _h) = tup;
    // note
    let (u, ..) = tup;
    println!("u is --> {u}");
    println!("tup is ==> {tup:?}"); // note the pretty print :?

    //6. Another way to access the values in a tuple is as follows.
    let c = tup.2;
    print!("c is --> {c}\n");

    /*  ========= 15/09/2022 Review Questions for next day ==========
    1. What case does Rust use for variables and functions?
    2. Are type annotations required for all parameters?
    3. What is the dif between statements and expressions in Rust?
    4. In python, x = y = 6 is valid code. How about Rust? Explain!
    */
}
