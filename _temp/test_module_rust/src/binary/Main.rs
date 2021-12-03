use std::time::Duration;
use std::time::SystemTime;

fn hanoi(n:i32, start:i32, work:i32, target:i32) {
    if(n==1){
        println!("%c에서 원반 %d를 %c로 옮김 \n")
    }
}

fn main() {
    let mut a = 551i32;
    let mut b = &a;
    println!("{}", a);

    let michael = User {
        id: 0,
        name: String::from("Michael"),
        is_activated: true
    };

    let mut tost = User {
        id: 0,
        name: "tost".parse().unwrap(),
        is_activated: true
    };

    tost.id = 2;

    println!("id: {}, name: {}", michael.id, michael.name);
    println!("id: {}, name: {}", tost.id, tost.name);
}
struct User {
    id: i32,
    name: String,
    is_activated: bool
}

#[derive(Debug)]
struct User1 {
    id: i32,
    name: String,
    is_activated: bool
}
