// from here
//https://levelup.gitconnected.com/rust-option-vs-result-when-to-use-what-e73e82612cb0

fn main() {
    println!("{:?}", prog_return_string());
    println!("{:?}", prog());

    let result_return_option: Option<f64> = return_option(1.0, 0.0);
    println!("result_return_option -> {:?}", result_return_option);

    match result_return_option {
        Some(x) => println!("Result of option is :{}", x),
        None => println!("Cannot divide by 0"),
    }
}

fn prog_return_string() -> String {
    // println!("{}","Hallo prog");
    return String::from("Prog return String ");
}

fn prog() -> String {
    String::from("Hello Prog")
}

fn return_option(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}
