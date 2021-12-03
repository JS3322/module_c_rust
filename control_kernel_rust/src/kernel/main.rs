/// * @Process: complete
/// * @Project_Name: module
/// * @Package_Name: kernel
/// * @Made_By: JS
/// * @The_creation_time: --
/// * @File_Name: main.rs
/// * @Version : rustc 1.57.0-nightly
/// * @contents: -

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    blog_os::init();

    // 새로 추가
    let ptr = 0xdeadbeaf as *mut u32;
    unsafe {
        *ptr = 42;
    }

    // 동일
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    blog_os::hlt_loop();
}
