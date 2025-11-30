fn main() {
    another_function();
    another_function_2(42);
    print_labeled_measurement(5, 'h');

    let x = five();
    println!("The value of x is: {}", x);

}

fn another_function() {
    println!("Another function.");  // 別の関数
}

fn another_function_2(x: i32) {
    println!("The value of x is: {}", x);   // xの値は{}です
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}