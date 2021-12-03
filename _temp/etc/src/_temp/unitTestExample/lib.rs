// unitTest --nocapture println output
// cargo test -- --nocapture

// unittest thread 1 test
// cargo test -- --test-threads=1

// tests folder can be omitted
//#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("###it works!###");
    }
}
