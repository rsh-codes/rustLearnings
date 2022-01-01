fn main() {
    println!("Hello, world!");

    another_func();

    let x = plus_one(5);
    println!("Value of x is: {}", x)
}

fn another_func(){
    println!("Another func!");
}

fn plus_one(x: i32)-> i32{
    x+1
}