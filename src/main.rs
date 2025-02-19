mod shapes;

fn main() {
    println!("Hello, world!");
    println!("I'm a Rustacean!");
    let x = 5 + 5;
    println!("x = {} before the mutate_x_val fn", x);
    mutate_x_val(x);
    println!("x = {} after the mutate_x_val function", x);
    // borrow x
    let mut x = 5;
    println!("x = {} before the mutate_x_ptr function", x);
    mutate_x_ptr(&mut x);
    println!("x = {} after the mutate_x_ptr function", x);
}

// Function to add 1 to x no return
fn mutate_x_ptr(x: &mut i32) {
    *x += 1;
    println!("x = {} in the mutate_x_ptr function", x);
}
fn mutate_x_val(mut x: i32) {
    x += 1;
    println!("x = {} in the mutate_x _val function", x);
}
#[cfg(test)]
mod tests {
    use super::*;
    use shapes::models::{Circle, Rectangle, Shape};

    #[test]
    fn test_mutate_x_val() {
        let x = 10;
        mutate_x_val(x);
        // x should remain unchanged because mutate_x_val takes x by value
        assert_eq!(x, 10);
    }

    #[test]
    fn test_mutate_x_ptr() {
        let mut x = 10;
        mutate_x_ptr(&mut x);
        // x should be incremented by 1
        assert_eq!(x, 11);
    }

    #[test]
    fn test_circle_area() {
        let circle = Circle::new(1.0).unwrap();
        assert_eq!(circle.area(), std::f64::consts::PI);
    }

    #[test]
    fn test_circle_perimeter() {
        let circle = Circle::new(1.0).unwrap();
        assert_eq!(circle.perimeter(), 2.0 * std::f64::consts::PI);
    }

    #[test]
    fn test_rectangle_area() {
        let rectangle = Rectangle::new(2.0, 3.0).unwrap();
        assert_eq!(rectangle.area(), 6.0);
    }

    #[test]
    fn test_rectangle_perimeter() {
        let rectangle = Rectangle::new(2.0, 3.0).unwrap();
        assert_eq!(rectangle.perimeter(), 10.0);
    }

    #[test]
    fn test_circle_new_err() {
        let circle = Circle::new(-1.0);
        assert!(circle.is_err());
    }

    #[test]
    fn test_rectangle_new_err() {
        let rectangle = Rectangle::new(2.0, -3.0);
        assert!(rectangle.is_err());
    }
}
