
#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn is_larger(&self, other: &Rectangle) -> bool {
         (self.width * self.height) > (other.width * other.height)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another_test() {
        panic!("This test will fail");
    }

    #[test]
    fn rect_is_larger() {
        let larger = Rectangle { width: 8, height: 9, };
        let smaller = Rectangle { width: 4, height: 2, };

        assert!(larger.is_larger(&smaller));
    }
}


