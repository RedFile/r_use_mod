use r_use_mod::{add_fn,is_even_fn,data_utils};//使用 lib 中的 mod

fn main() {
    let x=5;
    let y=10;
    println!("x and y sum is :{}",add_fn(x,y));
    println!("x is even ? {:?}",is_even_fn(x));
    data_utils::data_utils_imp::print_fn("hello world");
    data_utils::do_something_to_number();
}
