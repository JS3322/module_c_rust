

use std::time::SystemTime;

#[no_mangle]
pub extern fn hello() {
    println!("Hello________Test");
}

#[no_mangle]
pub extern fn hello_check() -> &'static str {

    println!("rust println : Hello____check");
    return "hello check!";
}

#[no_mangle]
pub extern fn sys_out() {
    let a = "check1";
    let b = "check2";
    let sys_time = SystemTime::now();
    let new_sys_time = SystemTime::now();
    println!("{}{:?}", a, sys_time);
    println!("{}{:?}", b, new_sys_time);
}

#[no_mangle]
pub extern fn add(a: i64, b: i64) -> i64 {
    return a + b;
}

#[no_mangle]
pub extern fn add_int(a: i32, b: i32) -> i32 {
    return a + b;
}

#[no_mangle]
pub extern fn fibonacci(x:i32) -> i32 {
    if x == 0 {return 0;}
    else if x == 1 {return 1;}
    else {return fibonacci(x-1)+fibonacci(x-2);}
}

#[no_mangle]
pub extern fn array_output(a:u64, b:u64, c:u64, d:u64, e:u64, f:u64) {
    let array_a = [a, b, c];
    let array_b = [d, e, f];

    let min = array_a.min(array_b);
    assert_eq!(min, array_b);
}

