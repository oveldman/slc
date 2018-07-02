#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate rocket;
extern crate rocket_contrib;

use std::path::Path;
use rocket_contrib::Template;
use rocket::response::NamedFile;

#[derive(Serialize)]
struct ViewModel {
    name: String,
    type_page: String,
}

#[get("/")]
fn index() -> Template {
    let model = ViewModel {
        name: "Gulden".to_string(),
        type_page: "index".to_string(),
    };

    Template::render("index", &model)
} 

#[get("/cv")]
fn cv() -> Template {
    let model = ViewModel {
        name: "Gulden".to_string(),
        type_page: "cv".to_string(),
    };

    Template::render("cv", &model)
} 

#[get("/projects/<leerjaar>")]
fn projects(leerjaar: u8) -> Template {
    let model = ViewModel {
        name: "Gulden".to_string(),
        type_page: "projects".to_string(),
    };

    let set_year: String = "projects".to_string() + &leerjaar.to_string();

    Template::render(set_year, &model)
} 

#[get("/leerdoelen/<leerjaar>")]
fn leerdoelen(leerjaar: u8) -> Template {
    let model = ViewModel {
        name: "Gulden".to_string(),
        type_page: "leerdoelen".to_string(),
    };

    let set_year: String = "leerdoel".to_string() + &leerjaar.to_string();

    Template::render(set_year, &model)
}

#[get("/contact")]
fn contact() -> Template {
    let model = ViewModel {
        name: "Gulden".to_string(),
        type_page: "contact".to_string(),
    };

    Template::render("contact", &model)
}  


#[get("/content/<file_type>/<file>")]
fn content_files(file_type: String, file: String) -> Option<NamedFile> {
    let standard_location: String = String::from("templates/content/");
    let type_location: String = standard_location + &file_type;
    NamedFile::open(Path::new(&type_location).join(file)).ok()
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index, cv, projects, leerdoelen, contact, content_files])
    .attach(Template::fairing())
    .launch();
}
