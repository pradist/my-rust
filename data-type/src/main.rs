fn main() {
    let x = 1;
    let y = 10.0;

    let z = x + y as i32;
    println!("{} + {} = {}", x, y, z);

    let msg = String::from("Hello, World!");
    let msg2 = "Hello, World!".to_string();
    let msg3 = "Hello, World!";
    let msg4 = format!("Point: {} {}", x, y);

    println!("{} {} {} {}", msg, msg2, msg3, msg4);
}
