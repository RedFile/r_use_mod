use super::print::my_print;//super 引用父级目录 data_utils

pub fn do_something_to_number(){
    my_print::print_fn(1);
}

pub fn do_something_to_string(){
    my_print::print_fn("hello");
}