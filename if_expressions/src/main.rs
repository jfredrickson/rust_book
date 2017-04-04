fn main() {
    one();
    two();
    three();
    four();
    five();
}

fn one() {
    let x = 5;
    if x == 5 {
        println!("x is five!");
    }
}

fn two() {
    let x = 5;
    if x == 5 {
        println!("x is five!");
    } else {
        println!("x is not five :(");
    }
}

fn three() {
    let x = 5;
    if x == 5 {
        println!("x is five!");
    } else if x == 6 {
        println!("x is six!");
    } else {
        println!("x is not five or six :(");
    }
}

fn four() {
    let x = 5;
    // this works because 'if' is an expression that produces a value.
    // an 'if' without an 'else' always produces a ().
    let y = if x == 5 {
        10
    } else {
        15
    };
    println!("y is {}", y);
}

fn five() {
    let x = 5;
    let y = if x == 5 { 10 } else { 15 };
    println!("y is {}", y);
}
