fn main() {
    // Static lifetimes
    let s: &'static str = "Let’s Get Rusty!";
}


// Lifetimes in function signatures
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetimes in struct definitions
struct User<'a> {
    full_name: &'a str,
}
