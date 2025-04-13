//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

// build.rs
fn main() {
    // ---------------------- 处理 tests7 的配置 ----------------------
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // 设置环境变量 TEST_FOO 为当前时间戳（格式：cargo:rustc-env=TEST_FOO=值）
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // ---------------------- 处理 tests8 的配置 ----------------------
    // 启用 "pass" 特性（格式：cargo:rustc-cfg=feature="pass"）
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
