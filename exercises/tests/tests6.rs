// tests6.rs
//
// In this example we take a shallow dive into the Rust standard library's
// unsafe functions. Fix all the question marks and todos to make the test
// pass.
//
// Execute `rustlings hint tests6` or use the `hint` watch subcommand for a
// hint.


struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
///
/// The `ptr` must contain an owned box of `Foo`.
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    // 将原始指针转换为 Box<Foo>
    let mut ret: Box<Foo> = unsafe { Box::from_raw(ptr) };
    // 修改 b 字段以满足测试断言
    ret.b = Some("hello".to_owned());
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let data = Box::new(Foo { a: 1, b: None });

        let ptr_1 = &data.a as *const u128 as usize;
        // SAFETY: 传递一个拥有所有权的 Box<Foo>
        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) };
        let ptr_2 = &ret.a as *const u128 as usize;

        assert_eq!(ptr_1, ptr_2);
        assert_eq!(ret.b, Some("hello".to_owned()));
    }
}