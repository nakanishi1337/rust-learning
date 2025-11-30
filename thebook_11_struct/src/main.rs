fn main() {
    example();
    example2();
    example3();
    example4();
    example5();
    example6();
    example7();
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn example() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
}

fn example2() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}

fn example3(){
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

}

fn example4(){
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

}

fn example5() {

    fn area(width: u32, height: u32) -> u32 {
    width * height
    }

    let width1 = 30;
    let height1 = 50;

    println!(
        // 長方形の面積は、{}平方ピクセルです
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn example6() {

    struct Rectangle {
        width: u32,
        height: u32,
    }
    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }


    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}   

fn example7(){
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle { width: 30, height: 50 };

    // rect1は{}です
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

}