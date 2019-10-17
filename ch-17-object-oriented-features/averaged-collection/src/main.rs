// "OO programs are made up of objects. An object packages both data and the
// procedures that operate on that data. The procedures are typically called
// methods or operations." (GoF book)

// Using rust in the good, old object-oriented tradition; AveragedCollection
// mimics a C++/Java class, as instances of this struct will have data and
// procedures associated with them.

mod my_collections {
    // NOTE: All fields of structs from other modules are private. Given that
    // AveragedCollection is in a module, code outside this module will not have
    // access to the fields - but code inside this module, does.
    pub struct AveragedCollection {
        list: Vec<i32>,
        average: f64,
    }

    impl AveragedCollection {
        pub fn new() -> Self {
            AveragedCollection {
                list: Vec::new(),
                average: 0.0,
            }
        }

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

    pub fn i_can_access_members () {
        let mut c = AveragedCollection { list: Vec::new(), average: 0.0 };
        c.add(10);
        c.add(5);
        println!("Average, field: {}", c.average); // this will work - can access fields from inside the module
    }
}


fn main() {
    let mut c = my_collections::AveragedCollection::new();
    c.add(10);
    c.add(5);
    println!("Average: {}", c.average());
    // println!("Average field: {}", c.average); // this will not work because fields are private to code outside the module
    my_collections::i_can_access_members(); // call a function that is inside the module - it does have access to members
}
