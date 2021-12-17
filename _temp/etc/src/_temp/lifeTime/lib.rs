const CONST: i64 = 10;

fn takes_static<T: ?Sized>(v: &'static T) -> &'static T {
    v
}

fn main() {
    println!("Takes static : {}", takes_static(&CONST));
    println!("Takes static : {}", takes_static("some literal"));
}
