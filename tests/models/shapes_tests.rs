#[cfg(test)]
mod tests {
    use prodcast::models::shapes::{Circle, Rectangle, Shape};

    /// Tests the area function for a unit circle
    #[test]
    fn test_circle_area() {
        let circle = Circle::new(1.0).unwrap();
        assert_eq!(circle.area(), std::f64::consts::PI);
    }

    /// Tests the perimeter function for a unit circle
    #[test]
    fn test_circle_perimeter() {
        let circle = Circle::new(1.0).unwrap();
        assert_eq!(circle.perimeter(), 2.0 * std::f64::consts::PI);
    }

    /// Tests the area function for a 2x3 rectangle
    #[test]
    fn test_rectangle_area() {
        let rectangle = Rectangle::new(2.0, 3.0).unwrap();
        assert_eq!(rectangle.area(), 6.0);
    }

    /// Tests the perimeter function for a 2x3 rectangle
    #[test]
    fn test_rectangle_perimeter() {
        let rectangle = Rectangle::new(2.0, 3.0).unwrap();
        assert_eq!(rectangle.perimeter(), 10.0);
    }

    /// Tests that Circle::new returns an error for a negative radius
    #[test]
    fn test_circle_new_err() {
        let circle = Circle::new(-1.0);
        assert!(circle.is_err());
    }

    /// Tests that Rectangle::new returns an error for a negative height
    #[test]
    fn test_rectangle_new_err() {
        let rectangle = Rectangle::new(2.0, -3.0);
        assert!(rectangle.is_err());
    }
}
