#[derive(PartialEq,Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // Vec implements IntoIter, which means it can be iterated over, but
    // an iterator must still be explicitly requested. Unlike other languages,
    // one cannot call iterator methods directly on the object
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("foo") },
        Shoe { size: 11, style: String::from("bar") },
        Shoe { size: 10, style: String::from("baz") },
    ];

    // note: shoes_in_my_size takes ownership of shoes ...
    let my_size = shoes_in_my_size(shoes, 10);
    // ... so this would not work: (borrow of moved value)
    //println!("{:?}", shoes);

    assert_eq!(
        my_size,
        vec![
            Shoe { size: 10, style: String::from("foo") },
            Shoe { size: 10, style: String::from("baz") },
        ]
    )
}

fn main() {
    println!("Hello, world!");
}
