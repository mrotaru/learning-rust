// Iterator trait: https://doc.rust-lang.org/std/iter/trait.Iterator.html
// General information, including how to implement an iterator: https://doc.rust-lang.org/std/iter/index.html

fn main() {
}

#[test]
fn consuming() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); // this consumes

    // `sum` takes ownership of the iterator; "consumes" it
    // because it repeatedly calls "next" on it. The consumed
    // iterator cannot be used afterwards, because `sum`
    // "swallowed" it (took ownership, didn't return it)
    // TO_GROK: why can't the type be inferred ?
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

#[test]
fn iterator_adaptors() {
    let v1 = vec![1, 2, 3];

    // iterators are lazy; this will not do anything:
    v1.iter().map(|x| x + 1);

    // one way of actually invoking the iterator is `collect`:
    // TO_GROK: again, type not inferred, ugly...
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}