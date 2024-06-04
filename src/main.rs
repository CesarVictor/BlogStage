#[macro_use] extern crate rocket;



#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/")]
fn about() -> &'static str {
    "About me"
}

#[get("/")]
fn contact() -> &'static str {
    "Contact me"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/about", routes![about])
        .mount("/contact", routes![contact])
        .mount("/posts", routes![posts])
}
