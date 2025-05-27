fn main() {
    //Variables are immutable
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    //Constants cannot have the mut tag, and data type must be notated
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; <- gives unused variable error

    //Shadowing variables
    let y = 10;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}"); //22
    }

    println!("The value of y is: {y}"); // 11

    let spaces = "   "; //str
    let spaces = spaces.len(); // num, we can save space instead of creating new variables for the string and number variable by reusing let

    /*
    Cannot do this with mut:
    let mut spaces = "   ";
    spaces = spaces.len()

    gives "expected &str, found usize"
    */

    //Data Types
    /*
    
    */

    let guess: u32 = "42".parse().expect("Not a number!");
}
