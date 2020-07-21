pub trait Messenger {
    fn send_message(&self, message: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}



fn main() {
    println!("Hello, world!");
}
