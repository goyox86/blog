use rocket::Rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
