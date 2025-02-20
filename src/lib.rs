pub mod shapes;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use shapes::models::{Circle, Rectangle, Shape};

/// Greet the user
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    hello();
    format!("Hello {}!", &name)
}

/// Health check endpoint
async fn healthz() -> impl Responder {
    HttpResponse::Ok().finish()
}

/// Hello world function
fn hello() {
    // println! is a macro that prints to the console
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

/// Run the server
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/healthz", web::get().to(healthz))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

// Function to add 1 to x as pointer no return
pub fn mutate_x_ptr(x: &mut i32) {
    *x += 1;
    println!("x = {} in the mutate_x_ptr function", x);
}

// Function to add 1 to x as val no return
pub fn mutate_x_val(mut x: i32) {
    x += 1;
    println!("x = {} in the mutate_x _val function", x);
}
