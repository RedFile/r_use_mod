// mod.rs为入口文件， 下面用mod声明会先去同文件夹下查找同名文件，如果没有则看是否有满足条件   的同名文件夹
// 例如 add 文件夹下没有print.rs 则查找是否有print文件夹并且文件夹下有mod.rs。
mod print;
mod do_something;

// 同一个模块文件夹下，并且在入口文件使用，所以这里应self,直接使用当前文件夹下的print.rs也行
// use self::print::my_print::print_fn;
pub use do_something::do_something_to_number;//通过do_something 文件引用 do_something_to_number
pub use do_something::do_something_to_string;

pub mod data_utils_imp{
    pub use super::print::my_print::print_fn;//通过data_utils_imp 文件引用 print_fn
}