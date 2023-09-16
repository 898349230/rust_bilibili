// 结构体中使用泛型
struct Point<T> {
    x: T,
    y: T,
}

// T 放在 impl 后，表示在类型T上实现方法
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let number_list = [1,4,67,43,88,98,15];
    let result = largest(&number_list);
    println!("result = {}", result);
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
// 方法中使用泛型
// fn largest2<T >(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// } 

