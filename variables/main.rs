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

}
