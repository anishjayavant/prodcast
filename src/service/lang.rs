/// Some basic language features
use crate::models::shapes::{Circle, Rectangle, Shape};

// Function to add 1 to x as pointer no return
pub fn mutate_x_ptr(x: &mut i32) {
    *x += 1;
    tracing::info!("x = {} in the mutate_x_ptr function", x);
}

// Function to add 1 to x as val no return
pub fn mutate_x_val(mut x: i32) {
    x += 1;
    tracing::info!("x = {} in the mutate_x _val function", x);
}

/// Hello world function
pub fn hello() {
    tracing::info!(
        "Rust greet processing.. be prepared for a greeting with some lang basics experimentation!"
    );
    tracing::info!("Hello, world!");
    tracing::info!("I'm a Rustacean!");
    let x: i32 = 5;
    tracing::info!("x = {} before the mutate_x_val fn", x);
    mutate_x_val(x);
    tracing::info!("x = {} after the mutate_x_val function", x);
    // borrow x
    let mut x = 5;
    tracing::info!("x = {} before the mutate_x_ptr function", x);
    mutate_x_ptr(&mut x);
    tracing::info!("x = {} after the mutate_x_ptr function", x);

    // try some stuff wth shapes
    let circle = Circle::new(1.0).unwrap();
    tracing::info!("Circle area: {}", circle.area());
    tracing::info!("Circle perimeter: {}", circle.perimeter());

    let rect = Rectangle::new(2.0, 3.0).unwrap();
    tracing::info!("Rectangle area: {}", rect.area());
    tracing::info!("Rectangle perimeter: {}", rect.perimeter());
}
