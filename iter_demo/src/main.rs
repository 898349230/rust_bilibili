fn main() {

    // next() 方法：每次返回迭代器中的一项，返回结果包裹在Some()中， 迭代结束返回None()
    // iter() 方法：在不可变引用上创建迭代器
    // into_tert() 方法：创建的迭代器会获得所有权
    // iter_mut() 方法：迭代可变的引用


    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("iter val = {}", val);
    }

    let v1_iter2 = v1.iter();
    // sum 方法 耗尽迭代器， 取得迭代器的所有权，通过反复调用next，遍历所有元素
    let sum: i32 = v1_iter2.sum();
    println!("sum = {}", sum);

    let v2 = vec![1,2,3];
    let v2: Vec<_> = v2.iter().map(|x| x+1).collect();
    assert_eq!(v2, vec![2,3,4]);
}
