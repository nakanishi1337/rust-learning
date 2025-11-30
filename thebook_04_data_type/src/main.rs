fn main() {
    learn_float();
    learn_operation();
    learn_boolean();
    learn_character();
    learn_tuple();
}

fn learn_float() {
    // float
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
}

fn learn_operation(){
    // addition
    let sum = 5.0 + 10.0;
    println!("The sum is: {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The difference is: {}", difference);

    // multiplication
    let product = 4.0 * 30.0;
    println!("The product is: {}", product);

    // division
    let quotient = 7.0 / 2.0;
    println!("The quotient is: {}", quotient);
}

fn learn_boolean() {
    // boolean
    let t = true;
    let f: bool = false; // with explicit type annotation
}

fn learn_character() {
    // character
    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}

fn learn_tuple() {
    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // destructuring
    println!("The value of y is: {}", y);
    
    // accessing tuple elements
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
}

fn learn_array() {
    // array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("The first element is: {}", first);
    println!("The second element is: {}", second);
}