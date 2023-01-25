//#![feature(proc_macro_hygiene, decl_macro)]

use crate::static_generate_invoice;
use crate::InvoiceDataJson;
use figment::{
    providers::{Format, Toml},
    Figment,
};
use rocket::http::{ContentType, Status};
use rocket::serde::json::Json;
use rocket::{launch, post, routes};
use std::borrow::Borrow;

#[post("/", data = "<invoice>")]
async fn index(invoice: Json<InvoiceDataJson>) -> (Status, (ContentType, Vec<u8>)) {
    let pdf = static_generate_invoice(invoice.0.borrow()).await.unwrap();
    (Status::Ok, (ContentType::PDF, pdf))
}

#[launch]
pub fn rocket() -> _ {
    let figment =
        Figment::from(rocket::Config::default()).merge(Toml::file("./Rocket.toml").nested());
    rocket::custom(figment).mount("/", routes![index])
}
