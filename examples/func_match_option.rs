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
        None
    } else {
        Some("Das ist ein großzügiger Tipp!".to_string())
    }
}

pub fn prog() -> () {
    // println!("Hello, example!");

    let tip = 30;
    match get_waiter_comment(tip) {
        Some(comment) => println!("Comment => {}", comment),
        None => println!("None = null"),
    }
}

fn main() {

prog();

}




#[cfg(test)]
mod tests {

    #[test]
    fn test_init() {
        assert_eq!(main, 4);
    }
}

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("your-binary-name")?;

    //cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}