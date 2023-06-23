// cd ~
// cargo new test_result_option_waiter
// cd test_result_option_waiter
// mkdir examples
// cp src/main.rs examples/func_match_option.rs
// cargo build --example func_match_option
// cargo run --example func_match_option

// from here
// https://blog.logrocket.com/ditching-try-catch-and-null-checks-in-rust/

//use std::{env, string};
//use std::path::Path;
//use std::ffi::OsStr;

pub fn get_waiter_comment(tip_percentage: u32) -> Option<String> {
    if tip_percentage <= 21 {
        Some("to small".to_string())
    } else {
        Some("Das ist ein guter Tipp!".to_string())
    }
}

pub fn prog() -> String {
    let tip: u32 = 1;
    //let result;
    match get_waiter_comment(tip) {
        Some(comment) => comment,
        None => String::from("haha"),
    }
}

fn main() {
    println!("{}", prog());
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_init() {
        assert_eq!(4, 4);
    }
}
