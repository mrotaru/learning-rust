fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    // fol loop automatically takes ownership of v_iter and
    // makes it mutable behind the scenes. This is needed because
    // v_iter keeps some internal state (current item) which needs
    // to be kept up to date
    for val in v1_iter {
        println!("Got: {}", val);
    }

    // when invoking `next` manually, we need to explicitly
    // make the iterator mutable
    let v2 = vec![1, 2, 3];
    let mut v2_iter = v2.iter();
    assert_eq!(v2_iter.next(), Some(&1));
    assert_eq!(v2_iter.next(), Some(&2));
    assert_eq!(v2_iter.next(), Some(&3));
    assert_eq!(v2_iter.next(), None);

    // iter vs into_iter: https://stackoverflow.com/a/34745885/447661
    // (TODO: grok)
}
