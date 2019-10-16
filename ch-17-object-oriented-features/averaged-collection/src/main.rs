// "OO programs are made up of objects. An object packages both data and the
// procedures that operate on that data. The procedures are typically called
// methods or operations." (GoF book)

// Using rust in the good, old object-oriented tradition; AveragedCollection
// mimics a C++/Java class, as instances of this struct will have data and
// procedures associated with them.

// NOTRE: All fields should be private, according to the book - unless I
// misunderstood something. But it doesn't seem to be the case in current rust.
// Also verified in the official playground, and fields don't seem to be
// private:
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=e1709ccf8727816f0ec9309599f8c8eb
// online book: https://doc.rust-lang.org/stable/book/ch17-01-what-is-oo.html
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}


fn main() {
    let mut c = AveragedCollection { list: Vec::new(), average: 0.0 };
    c.add(10);
    c.add(5);
    println!("Average: {}", c.average());
    println!("Average field: {}", c.average); // this should not work, but it does - fields are not private
}
