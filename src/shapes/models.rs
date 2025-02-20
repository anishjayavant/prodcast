//! This module contains the models for the shapes

/// Represents a circle
pub struct Circle {
    radius: f64,
}

/// Represents a rectangle
pub struct Rectangle {
    width: f64,
    height: f64,
}

/// Represents a shape that defines area and perimeter functions
#[allow(dead_code)]
pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

/// Constructor for Circle with validations
#[allow(dead_code)]
impl Circle {
    pub fn new(radius: f64) -> Result<Circle, String> {
        if radius <= 0.0 {
            return Err("Radius must be greater than 0".to_string());
        }
        Ok(Circle { radius })
    }
}

/// Constructor for Rectangle with validations
#[allow(dead_code)]
impl Rectangle {
    pub fn new(width: f64, height: f64) -> Result<Rectangle, String> {
        if width <= 0.0 || height <= 0.0 {
            return Err("Width and height must be greater than 0".to_string());
        }
        Ok(Rectangle { width, height })
    }
}

/// Implement the Shape trait for Circle
impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

/// Implement the Shape trait for Rectangle
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

// Module unit tests
#[cfg(test)]
mod tests {
    use super::*;

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
