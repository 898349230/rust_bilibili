pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct Guess {
    value: u32
}

impl Guess {
    pub fn new(x: u32) -> Guess {
        if (x < 1 || x > 100) {
            panic!("x is bigger 100, x = {}", x);
        }
        Guess { value: x }
    }
}

// 单元测试，只有运行 cargo test 才编译和运行代码
// cargo build 不会编译执行
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_panic(){
        panic!("error 了");
    }    
    
    #[test]
    fn test_assert(){
        // assert!(3 < 5);
        // assert_eq!(3 , 5);
        // assert_ne!(3 , 5);

        // 自定义异常信息，允许使用占位符，因为第二个参数返回的是 format!()
        let result = String::from("哈哈");
        assert!(3 > 5, "这个是自定义的信息 => {}",result);
    }

    #[test]
    // 发生 panic 时测试通过
    #[should_panic]
    // 发生 panic 时,并且信息包含"aaa"时测试通过
    // #[should_panic(expected="aaa")]
    fn greater_than_100(){
        // 发生panic时通过
        Guess::new(200);
    }    
    #[test]
    // 忽略测试
    #[ignore = "reason 121"]
    fn test_ignore(){
        // 发生panic时通过
        Guess::new(200);
    }

// cargo test -- --test-threads=1  设置运行线程数
// cargo test -- --show-output 输出 println 内容，想在成功的测试中看到打印的内容
// cargo test greater_than_100   只运行 greater_than_100 测试方法
// cargo test tes   运行 tes 开头的测试方法
// cargo test -- --ignored  单独运行 ignore 的测试

}
