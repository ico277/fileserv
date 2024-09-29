#[macro_use] extern crate rocket;

#[get("/")]
async fn index() -> &'static str {
    "meow"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
