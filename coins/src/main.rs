/*
install: gnuplot, postgres, libssl-dev
*/

#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate chrono;
extern crate rocket;
extern crate rocket_contrib;
extern crate coins;
extern crate curl;
extern crate gnuplot;
extern crate postgres;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod gulden;
mod database;
mod request;
mod price_updater;
mod reader;
mod graph;

use std::path::Path;
use std::thread;
use gulden::Market;
use database::client::CryptoCoins;
use rocket_contrib::Json;
use rocket_contrib::Template;
use rocket::response::NamedFile;

#[derive(Serialize)]
struct ViewModel {
    name: String,
    coin_image: String,
}

#[get("/", format = "text/html")]
fn index() -> &'static str {
    "Hello, world!"
}


#[get("/gulden")]
fn gulden_page() -> Template {
    let gulden: CryptoCoins = database::client::get_all_price_gulden();
    let image_location: String = graph::make_plot(gulden);
    let model = ViewModel {
        name: "Gulden".to_string(),
        coin_image: image_location,
    };
    Template::render("index", &model)
} 

#[get("/gulden/getcurrentprice")]
fn gulden_getprice() -> Json<Market> {
   let response: Market = gulden::get_price();
   Json(response)
}

#[get("/coins/gethistoryprice")]
fn history_getprice() -> Json<CryptoCoins> {
   let response: CryptoCoins = database::client::get_all_price_gulden();
   Json(response)
}

#[get("/content/<file>")]
fn content_files(file: String) -> Option<NamedFile> {
    NamedFile::open(Path::new("templates/content").join(file)).ok()
}

fn main() {
    thread::spawn(move || {
        database::client::update();
        price_updater::update();
    });
    
    rocket::ignite()
    .mount("/", routes![index, gulden_getprice, history_getprice, gulden_page, content_files])
    .attach(Template::fairing())
    .launch();
}