// 导入被测试库
use adder;
mod common;

#[test]
fn it_adds_two () {
    assert_eq!(4, adder::add(1, 3));

    // cargo test --test another_test  只运行一个集成测试文件
    // common 目录下的文件不会别认为是集成测试文件
    // 如果项目是 binary crate， 只含有 src/main.rs 没有 src/lib.rs
    // 不能在 tests 目录下创建集成测试，无法把 main.rs 的函数导入作用域
}