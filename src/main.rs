mod shapes;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use shapes::models::{Circle, Rectangle, Shape};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    hello();
    format!("Hello {}!", &name)
}

fn hello() {
    println!("Hello, world!");
    println!("I'm a Rustacean!");
    let x: i32 = 5;
    println!("x = {} before the mutate_x_val fn", x);
    mutate_x_val(x);
    println!("x = {} after the mutate_x_val function", x);
    // borrow x
    let mut x = 5;
    println!("x = {} before the mutate_x_ptr function", x);
    mutate_x_ptr(&mut x);
    println!("x = {} after the mutate_x_ptr function", x);

    // try some stuff wth shapes
    let circle = Circle::new(1.0).unwrap();
    println!("Circle area: {}", circle.area());
    println!("Circle perimeter: {}", circle.perimeter());

    let rect = Rectangle::new(2.0, 3.0).unwrap();
    println!("Rectangle area: {}", rect.area());
    println!("Rectangle perimeter: {}", rect.perimeter());
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
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
}
