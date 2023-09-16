use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io;
fn main() {

// 设置 RUST_BACKTRACE=1 环境变量可以显示详细调用链
// 为了获取带有调试信息的回溯，必须启用调试符号（不带 --release）
// panci 是不可恢复的错误
    // panic!("cra sh and abort!");

    // let result = read_username_from_file();
    let result = read_username_from_file3();

    match result {
        Ok(str) => println!(" str: {}", str),
        Err(e) => panic!(" 异常 {:?}", e)
    }

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("error create file {:?}",e),
            },
            other_error => panic!("Error opening file {:?}", other_error),
        },
    };

    // unwrap() 返回 Result 中的 Ok() 内的值 或者 Err 的 panic！
    let f2 = File::open("hello.txt").unwrap();
    // expect() 和 unwrap() 方法类似，不过可以自定义 Panci 中的内容
    let f3 = File::open("hello.txt").expect("无法打开文件");


}

/* 传播错误，返回 Result */
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        // 成功返回打开的文件
        Ok(file) => file,
        // 失败 函数返回 Err(e)
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    // 返回 match 表达式的返回值， 
    // 把文件的内容读取到 s 中
    match f.read_to_string(&mut s){
        // 成功返回
        Ok(_) => Ok(s),
        // 失败返回
        Err(e) => Err(e),
    }
}
/* 使用 ? 精简代码. ? 类型只能返回 Result 类型 */
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    // 语句后面使用  ? 代替下面的 match
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    // 使用 ? 代替下面的内容
    // match f.read_to_string(&mut s){
    //     // 成功返回
    //     Ok(_) => Ok(s),
    //     // 失败返回
    //     Err(e) => Err(e),
    // }

    Ok(s)
}
/* 链式调用 */
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}