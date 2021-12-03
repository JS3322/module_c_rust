fn test_1() {
    let hi = "hi";
    let mut count = 0;

    while count < 10 {
        println(fmt!("count: %?", count));
        count += 1;
    }
}

fn test_2() {
    static MONSTER_FACTOR: float = 57.8;
    let monster_size = MONSTER_FACTOR * 10.0;
    let monster_size: int = 50;

    let my_variable = 100;
    type MyType = int;
}

fn hello() {
    println!("Hello________Test");
}
fn sys_out() {
    let a = "check1";
    let b = "check2";
    let sys_time = SystemTime::now();
    let new_sys_time = SystemTime::now();
    let difference = new_sys_time.duration_since(sys_time)
        .expect("Clock may have gone backwards");
    println!("{}{:?}", a, sys_time);
    println!("{}{:?}", b, new_sys_time);
}
fn sys_out1() {
    let now = SystemTime::now();

    // we sleep for 2 seconds
    sleep(Duration::new(2, 0));
    match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("{}", elapsed.as_secs());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {:?}", e);
        }
    }
}
fn hello_check() {
    println!("Hello____check");
}
fn add(a: i64, b: i64) -> i64 {
    a + b
}
fn it_works() {
    assert_eq!(2 + 2, 4);
}
fn system_test() {

}
fn array_output(a:u64, b:u64, c:u64, d:u64, e:u64, f:u64) {
    let array_a = [a, b, c];
    let array_b = [d, e, f];

    let min = array_a.min(array_b);
    assert_eq!(min, array_b);
}
