// 导入被测试库
use adder;

#[test]
fn it_another_adds_two () {
    assert_eq!(4, adder::add(1, 3));
}