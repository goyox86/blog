use rocket::Rocket;
use rocket::Route;

use controllers::*;

pub fn draw() -> Vec<Route> {
    routes![
        posts_controller::new,
        posts_controller::create,
        posts_controller::index,
        posts_controller::show,
        posts_controller::edit,
        posts_controller::update,
        posts_controller::destroy,
    ]
}
