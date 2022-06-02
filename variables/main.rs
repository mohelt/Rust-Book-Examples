fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;
    Floating point?
   let x = 2.0; // f64
   let y: f32 = 3.0; // f32

//Numeric operations?
   // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
    // remainder
    let remainder = 43 % 5;

//Boolean type?
   let t = true;
    let f: bool = false; // with explicit type annotation

//Character type?
   let c = 'z';
   let z = 'ℤ';
   let heart_eyed_cat = '😻';

//Compound types?
//Arrays and tuples
let tup: (i32, f64, u8) = (500, 6.4, 1);

//Access tuple elements?
let x: (i32, f64, u8) = (500, 6.4, 1);
let five_hundred = x.0;
let six_point_four = x.1;
let one = x.2;

//Array type
let months = ["January", "February", "March", "April", "May", "June", "July",
          "August", "September", "October", "November", "December"];
let a: [i32; 5] = [1, 2, 3, 4, 5];
let first = a[0];

Parameters?
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

//What is a statement?
//Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value. Let’s look at some examples.Statements do not return values.

//Return values?
fn five() -> i32 {
    5
}

//Comments?
//hello world

//If statements?
   let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

//Elseif?
let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

//If in a let statement?
let number = if condition { 5 } else { 6 };

//Loop?
//The loop keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop
  loop {
        println!("again!");
    }

//Break out of loop?
//Break

//While loop?
   while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

//For loop?
   for element in a {
        println!("the value is: {}", element);
    }

}
