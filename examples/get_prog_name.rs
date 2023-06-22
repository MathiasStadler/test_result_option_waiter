// from here
// https://stackoverflow.com/questions/36848132/how-to-get-the-name-of-current-program-without-the-directory-part

// build 
// cargo run --package test_result_option_waiter --example get_prog_name

// run
// cargo run --package test_result_option_waiter --example get_prog_name
use std::env;
use std::path::Path;
use std::ffi::OsStr;

fn prog() -> Option<String> {
    env::args().next()
        .as_ref()
        .map(Path::new)
        .and_then(Path::file_name)
        .and_then(OsStr::to_str)
        .map(String::from)
}
fn prog_version_two() -> Option<String> {
    std::env::current_exe()
        .ok()?
        .file_name()?
        .to_str()?
        .to_owned()
        .into()
}

fn main() {
    println!("{:?}", prog());
    println!("{:?}", prog_version_two());
}
// Err : `Option<String>` cannot be formatted with the default formatter