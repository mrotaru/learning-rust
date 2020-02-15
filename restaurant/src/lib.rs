mod front_of_house {
    pub mod hosting {
        //fn add_to_waitlist() {} // function would still be private, even if module is public
        pub fn add_to_waitlist() {
            // `super` can be used as '..' - makes path relative to parent module
            // super::super::super::take_order(); // E: "too many initial super's"
            super::serving::take_order();
            println!("add_to_waitlist");
        }

        fn seat_at_table() {}
    }

    // for access by sibling modules, no need for 'pub' for module; but 'pub' is
    // still required for the individual functions.
    mod serving {
        pub fn take_order() {
            println!("take_order: This probably makes no sense...");
        }

        fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String, // public - `toast` can be changed by code outside the module
        seasonal_fruit: String, // ... but `seasonal_fruit` can't - private by default
    }

    // unlike structs, where a 'pub' doesn't mean access to fields, if an enum is 'pub',
    // all values are also 'pub'
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // two ways of accessing the same functionality:
    crate::front_of_house::hosting::add_to_waitlist(); // 'crate' used as '/' would be in the file system - denotes root
    front_of_house::hosting::add_to_waitlist();

    let mut meal = crate::back_of_house::Breakfast::summer("Rye"); // works - can change 'pub' field
    // meal.seasonal_fruit = String::from("blueberries"); // doesn't work - `seasonal_fruit` is private field
}
