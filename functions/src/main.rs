fn main() {
    foo();
    print_number(5);
    print_sum(5, 6);
    add_one(1);
    declaration_statement();
    early_return_1(5);
    early_return_2(5);
    // diverges();
    // let x: i32 = diverges();
    // let x: String = diverges();
    let f: fn(i32) -> i32 = plus_one;
    f(5);
    let f = plus_one;
    f(5);
}

fn foo() {
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn print_sum(x: i32, y: i32) {
    println!("sum is: {}", x + y);
}

fn add_one(x: i32) -> i32 {
    x + 1 // No semicolon for the return expression
}

fn declaration_statement() {
    // let mut y = 5;
    // let x = (y = 6); // x is () -- an empty tuple, which is what the expression (y = 6) returns.
}

fn early_return_1(x: i32) -> i32 {
    return x;

    // x + 1 // Never gets executed
}

fn early_return_2(x: i32) -> i32 {
    return x + 1; // Works, but is poor style. Better to just use the expression x + 1.
}

// fn diverges() -> ! {
    // panic!("This function never returns!");
// }

fn plus_one(i: i32) -> i32 {
    i + 1
}
