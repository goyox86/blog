use rocket::Rocket;

#[get("/posts/new")]
pub fn new() -> &'static str  {
    "posts#new"
}

#[post("/posts")]
pub fn create() -> &'static str  {
    "posts#create"
}

#[get("/posts")]
pub fn index() -> &'static str  {
    "posts#index"
}

#[get("/posts/<id>")]
pub fn show(id: i32) -> &'static str  {
    "posts#show"
}

#[get("/posts/<id>/edit")]
pub fn edit(id: i32) -> &'static str  {
    "posts#edit"
}

#[put("/messages/<id>")]
pub fn update(id: i32) -> &'static str  {
    "posts#update"
}

#[delete("/posts/new")]
pub fn destroy() -> &'static str {
    "posts#destroy"
}

