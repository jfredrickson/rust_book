//! # Comments
//!
//! This is a type of doc comment that applies to containing items (crates, modules, functions)
//! rather than the items following the comment. Commonly used in crate root lib.rs or modules
//! root mod.rs.
//!
//! Run 'rustdoc' to generate documentation from the comments in this file.

fn main() {
    // This is a line comment.
    add_one(5);
}

// Doc comments start with three slashes and support Markdown:

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, add_one(5));
/// # fn add_one(x: i32) -> i32 {
/// #     x + 1
/// # }
/// ```
fn add_one(x: i32) -> i32 {
    x + 1
}
