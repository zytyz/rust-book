
fn main() {

    //---------------- Varaibles ----------------------------------------------

    // Cannot assign to an immutable variable twice.
    // let x = 5; // ERROR
    let mut x = 5;
    println!("The value of x is: {x}");
    
    x = 6;
    println!("The value of x is: {x}");

    // const: always immutable
    // convention: Upper cases with underscore
    const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;

    // Shadowing
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // With shadowing, we can even change the type of the variables.
    let spaces = "   ";
    let spaces = spaces.len();
    
    // We can't do the same with reassigning a mutable
    // let mut spaces = "   ";
    // spaces = spaces.len(); // ERROR


    // ---------------- Data Types ---------------------------------------------
    // Scalar types:
    let x : i32 = 5;
    let x : f64 = 5.0;
    let x : bool = true;
    let x : char = 'a';
    let x : char = '😀';


    // Compound types:
    // tuples: Elements can have different types.
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    let five_hundred = tup.0; // access tuple data
    let six_point_four = tup.1;
    let one = tup.2;

    // array: Elements have to have the same type.
    // Fixed number of elements. Data stored on stack.
    let a : [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3 ; 5]; // Size 5. Values are all 3.
    // Would panic if index out of bounds
    let first = a[0];
    let second = a[1];


    //--------------- Functions -----------------------------------------------
    another_function(5);


    //---------- Statement and Expressions -----
    // Statements: instructions that perform actions and do not return values.
    // Expressions: evaluate to a resultant value.

    // let x = (let y = 6); // ERROR.

    let y = {
        let x = 3;
        x + 1
    }; // The expression would evaluate to 4
    println!("The value of y is: {y}");


    //--------- Functions with return values ----------------------------------
    let x = five();
    println!("The value of x is: {x}");


    //-------- Control Flow ---------------------------------------------------
    // if statements
    let number = 3;
    // if number // ERROR. Condition must be a bool.
    if number < 5 {
        println!("condition was true");
    } else if number < 10 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Using if in a let statement.
    // The types in all branches have to match.
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // loop statements
    // Return value from loops
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            // Put the return value after break.
            break counter * 2;
        }
    };

    println!("The result is {result}");
    
    // Loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                // breaks the outer loop.
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // while loop
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    // for loop
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }


}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

// functions with return values
fn five() -> i32 {
    5
    // 5; // ERROR. With a ";", this becomes a statement
}
