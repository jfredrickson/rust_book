fn main() {
    vectors();
    accessing_elements();
    out_of_bounds();
    iterating();
}

fn vectors() {
    // Vectors are implemented as Vec<T>, data allocated on heap, created with the vec! macro
    let v1 = vec![1, 2, 3, 4, 5]; // Vec<i32>
    let v2 = vec![0; 10]; // Vec<i32> of ten zeroes
    println!("{}", v1[0]);
    println!("{}", v2[0]);
}

fn accessing_elements() {
    let v = vec![1, 2, 3, 4, 5];
    println!("The third element of v is {}", v[2]);
    // index must be usize
    // let i: usize = 0;
    // let j: i32 = 0;
    // v[i]; // works
    // v[j]; // does not work
}

fn out_of_bounds() {
    let v = vec![1, 2, 3];
    // println!("Item 7 is {}", v[7]); // panic
    match v.get(7) {
        Some(x) => println!("Item 7 is {}", x),
        None => println!("Sorry, this vector is too short.")
    }
}

fn iterating() {
    let mut v = vec![1, 2, 3, 4, 5];

    for i in &v {
        println!("A reference to {}", i);
    }

    for i in &mut v {
        println!("A mutable reference to {}", i);
    }

    // note: cannot use the vector again after iterating by taking ownership
    for i in v {
        println!("Take ownership of the vector and its element {}", i);
    }
    // does not work now:
    // for i in v {
    //     println!("Take ownership of the vector and its element {}", i);
    // }
}
