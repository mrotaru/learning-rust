#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 2, width: 2 };
        let smaller = Rectangle { length: 1, width: 1 };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 2, width: 2 };
        let smaller = Rectangle { length: 1, width: 1 };
        let can_hold = smaller.can_hold(&larger);
        assert!(can_hold == false, "Was expecting false, got: {}", can_hold);
    }
}
