pub mod my_print{
    use std::fmt::Display;

    pub fn print_fn<T:Display>(x:T){
        println!("{}",x);
    }
}