mod add;
mod data_type;
pub mod data_utils;

pub use crate::add::add_fn;//crate 引用同级目录
pub use crate::data_type::is_even::is_even_fn;//通过 data_type 文件引用 data_type 文件夹中的文件
