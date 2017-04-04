fn main() {
    infinite_loop();
    while_loop();
    for_loop();
    enumeration();
    breaking();
    continuing();
    labels();
}

fn infinite_loop() {
    // loop {
    //     println!("Loop forever!");
    // }
}

fn while_loop() {
    let mut x = 5;
    let mut done = false;
    while !done {
        x += x - 3;
        println!("{}", x);
        if x % 5 == 0 {
            done = true;
        }
    }
}

fn for_loop() {
    // format:
    // for var in expression {
    //     code
    // }
    // the expression is an item that can be converted into an iterator using IntoIterator.
    for x in 0..10 {
        println!("{}", x);
    }
}

fn enumeration() {
    for (index, value) in (5..10).enumerate() {
        println!("index = {} and value = {}", index, value);
    }

    let lines = "hello\nworld".lines();
    for (linenumber, line) in lines.enumerate() {
        println!("{}: {}", linenumber, line);
    }
}

fn breaking() {
    let mut x = 5;
    loop {
        x += x - 3;
        println!("{}", x);
        if x % 5 == 0 { break; } // break out of the loop
    }
}

fn continuing() {
    for x in 0..10 {
        if x % 2 == 0 { continue; } // move on to next iteration
        println!("{}", x);
    }
}

fn labels() {
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 { continue 'outer; }
            if y % 2 == 0 { continue 'inner; }
            println!("x: {}, y: {}", x, y);
        }
    }
}
