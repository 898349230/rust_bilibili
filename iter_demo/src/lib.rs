#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size (shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|x| x.size == shoe_size).collect()
}

#[test]
fn filter_by_size (){
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("皮鞋1"),
        },
        Shoe {
            size: 12,
            style: String::from("皮鞋2"),
        },
        Shoe {
            size: 13,
            style: String::from("皮鞋3"),
        },
    ];

    let in_my_size = shoes_in_my_size(shoes, 13);
    assert_eq!(in_my_size, vec![Shoe {
        size: 13,
        style: String::from("皮鞋3"),
    },]);
}


struct Counter {
    count: u32,
}

impl Counter {
    fn new () -> Counter {
        Counter { count: 0 }
    }
}
// 自定义迭代器
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        }else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some((1)));
    assert_eq!(counter.next(), Some((2)));
    assert_eq!(counter.next(), Some((3)));
    assert_eq!(counter.next(), Some((4)));
    assert_eq!(counter.next(), Some((5)));
    assert_eq!(counter.next(), None);
}