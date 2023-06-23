// from here
//https://levelup.gitconnected.com/rust-option-vs-result-when-to-use-what-e73e82612cb0

fn main() {
    println!("{:?}", prog_return());
    println!("{:?}", prog());
}

fn prog_return() -> String {
    // println!("{}","Hallo prog");
    return String::from("Hallo Prog return ");
}

fn prog() -> String {
    String::from("Hello Prog")
}
