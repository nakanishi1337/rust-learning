fn main() {

    if_control(); // if文の例
    if_control2(); // if文の例2
    if_control3(); // if文の例3
    loop_control(); // ループ制御の例
    while_control(); // while文の例
    for_control(); // for文の例
}

fn if_control() {
    let number = 3;

    if number < 5 {
        println!("condition was true");       // 条件は真でした
    } else {
        println!("condition was false");      // 条件は偽でした
    }
}

fn if_control2() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4"); // 数は4で割り切れます
    } else if number % 3 == 0 {
        println!("number is divisible by 3"); // 数は3で割り切れます
    } else if number % 2 == 0 {
        println!("number is divisible by 2"); // 数は2で割り切れます
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_control3() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // numberの値は、{}です
    println!("The value of number is: {}", number);

}

fn loop_control() {
    let mut count = 0;

    loop {
        count += 1;
        println!("count is: {}", count);

        if count == 5 {
            break; // 5に達したらループを抜ける
        }
    }
}

fn while_control() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1; // numberを1ずつ減らす
    }

    println!("LIFTOFF!!!");
}

fn for_control() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // 0から4までの数値を出力
    for number in (1..=5).rev() {
        println!("{}!", number);
    }
}