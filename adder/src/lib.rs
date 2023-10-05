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

}
