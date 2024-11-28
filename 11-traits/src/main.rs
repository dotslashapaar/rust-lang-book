fn main() {
    println!("Hello, world!");
}

trait Animal {
    fn new(name: &'static str) -> Self;
    fn noise(&self) -> &'static str { "" }
}
struct Dog { name: &'static str }
    impl Dog {
        fn fetch() {   }
    }
impl Animal for Dog {
    fn new(name: &'static str) -> Dog {
        Dog { name: name }
    }
    fn noise(&self) -> &'static str {
        "woof!"
    }
    }
    