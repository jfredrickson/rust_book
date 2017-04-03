fn main() {
    bindings();
    patterns();
    type_annotations();
    mutability();
    initializing_bindings();
    scope_and_shadowing_1();
    scope_and_shadowing_2();
    scope_and_shadowing_3();
}

fn bindings() {
    let x = 5;
    println!("x:{}", x);
}

fn patterns() {
    let (x, y) = (1, 2); // LHS of a let statement is a pattern, not a variable name
    println!("x:{} y:{}", x, y);
}

fn type_annotations() {
    let x: i32 = 5; // 32-bit signed integer
    println!("x:{}", x);
}

fn mutability() {
    let mut x = 5;
    println!("x:{}", x);
    x = 10;
    println!("x:{}", x);
}

fn initializing_bindings() {
    // let x: i32;
    // println!("The value of x is: {}", x); // Causes an error
}

fn scope_and_shadowing_1() {
    let x: i32 = 17;
    {
        let y: i32 = 3;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    // println!("The value of x is {} and value of y is {}", x, y); // Doesn't work
}

fn scope_and_shadowing_2() {
    let x: i32 = 8;
    {
        println!("{}", x); // Prints "8"
        let x = 12;
        println!("{}", x); // Prints "12"
    }
    println!("{}", x); // Prints "8"
    let x = 42;
    println!("{}", x); // Prints "42"
}

fn scope_and_shadowing_3() {
    let mut x: i32 = 1;
    println!("{}", x);
    x = 7;
    println!("{}", x);
    let x = x;
    println!("{}", x);
    let y = 4;
    println!("{}", y);
    let y = "I can also be bound to text!";
    println!("{}", y);
}
