// cd ~
// cargo new test_result_option_waiter
// cd test_result_option_waiter
// mkdir examples
// cp src/main.rs examples/func_match_option.rs
// cargo build --example func_match_option
// cargo run --example func_match_option

// from here
// https://blog.logrocket.com/ditching-try-catch-and-null-checks-in-rust/

fn  get_waiter_comment  (  tip_percentage  :  u32  )   ->   Option  <  String  >   { 
    if  tip_percentage  <=   20   { 
        None 
    }   else   { 
        Some  (  "Das ist ein großzügiger Tipp!"  .  to_string  ()) 
    } 
} 

fn main() {
    println!("Hello, example!");

    let tip = 21;
    match get_waiter_comment  (  tip  )   { 
        Some  (  comment  )   =>  println!( "Comment => {}", comment  ),
        None   =>  println!("None = null") 
      }
}
