use std::collections::HashMap;

struct Cacher<T>
where
    T: Fn(&u32) -> u32,
{
    calculation: T,
    values: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(&u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> &u32 {
        if self.values.contains_key(&arg) {
            return self.values.get(&arg).unwrap()
        } else {
            let v = (self.calculation)(&arg);
            self.values.insert(arg, v);
            return self.values.get(&arg).unwrap()
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a + 1);
    let v1 = c.value(1);
    assert_eq!(*v1, 2);
    let v2 = c.value(2);
    assert_eq!(*v2, 3);
}

fn main() {}
