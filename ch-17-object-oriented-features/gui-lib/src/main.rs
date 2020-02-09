// using trait objects - contained elements can be of any type, as long as they implement the `Draw` trait
mod gui_lib_trait_object {
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }
}

// using generic type parameter - all contained elements must be of same type
mod gui_lib_generic_type_parameter {
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen<T: Draw> {
        pub components: Vec<T>,
    }

    impl<T> Screen<T> where T: Draw {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }
}

use gui_lib_trait_object::{Draw, Screen};

fn main() {
    #[derive(Debug)]
    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            println!("{:?}", &self);
        }
    }

    #[derive(Debug)]
    pub struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            println!("{:?}", &self);
        }
    }

    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("Button 1"),
            }),
            Box::new(SelectBox {
                width: 50,
                height: 10,
                options: vec![
                    String::from("Option 1"),
                    String::from("Option 2"),
                ],
            }),
        ]
    };

    screen.run();
}