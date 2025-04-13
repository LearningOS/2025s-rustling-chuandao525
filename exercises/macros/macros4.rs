// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.



#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };  // 添加分号
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };  // 添加分号（可选，但推荐保持统一）
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
