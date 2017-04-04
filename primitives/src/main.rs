fn main() {
    booleans();
    chars();
    numerics();
    arrays();
    slices();
    strs();
    tuples();
    functions();
}

fn booleans() {
    let x = true;
    let y: bool = false;
    println!("{} {}", x, y);
}

fn chars() {
    let x = 'x';
    let two_hearts = '💕'; // char represents a single Unicode scalar value (4 bytes, not 1 byte)
    println!("{} {}", x, two_hearts);
}

fn numerics() {
    // i8, i16, i32, i64: signed, fixed-size
    // u8, u16, u32, u64; unsigned, fixed-size
    // isize: signed, variable-size (depends on machine's underlying architecture)
    // usize: unsigned, variable-size (depends on machine's underlying architecture)
    // f32, f64: floating-point single and double precision
    let x = 42; // i32
    let y = 1.0; // f64
    println!("{} {}", x, y);
}

fn arrays() {
    // arrays have type [T; N]
    let a = [1, 2, 3]; // a: [i32; 3]
    // let mut m = [1, 2, 3]; // m: [i32; 3]
    // let a = [0; 20]; // a: [i32; 20] (initializes all elements to 0)
    println!("a has {} elements", a.len());
    let names = ["Graydon", "Brian", "Niko"]; // names: [&str; 3]
    println!("The second name is: {}", names[1]);
}

fn slices() {
    // a slice is a view into a data structure
    // slices allow access without copying
    // slices are not created directly, but from an existing binding
    // slices have a defined length
    // slices can be mutable or immutable
    // slices are represented internally as a pointer to the beginning of the data and a length
    // slices have type &[T]
    let a = [0, 1, 2, 3, 4];
    let complete = &a[..]; // a slice containing all elements in 'a'
    let middle = &a[1..4]; // a slice containing only the elements 1, 2, 3
    println!("elements in complete: {}", complete.len());
    println!("elements in middle: {}", middle.len());
}

fn strs() {
    // unsized type
    // useful when placed behind a reference like &str
    // look at &str as a "string slice"
}

fn tuples() {
    // an ordered list of fixed size
    let x = (1, "hello"); // 2-length tuple
    println!("{} {}", x.0, x.1); // tuple indexing to access fields in a tuple
    let x: (i32, &str) = (1, "hello"); // same as above with explicit type annotations
    println!("{} {}", x.0, x.1);
    let mut x = (1, 2);
    println!("{} {}", x.0, x.1);
    let y = (2, 3);
    println!("{} {}", y.0, y.1);
    x = y; // can assign one tuple to another if they have the same types and arity
    println!("{} {}", x.0, x.1);
    let (x, y, z) = (1, 2, 3); // destructuring let
    println!("x is {}, y is {}, z is {}", x, y, z);
}

fn functions() {
    fn foo(x: i32) -> i32 { x }; // a function of type fn(i32) -> i32
    let x: fn(i32) -> i32 = foo; // x is a function pointer
    x(5);
}
