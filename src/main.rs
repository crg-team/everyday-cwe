#[macro_use]
extern crate rocket;

use rocket::fs::{NamedFile};
use rocket_dyn_templates::{context, Template};
use std::path::{Path, PathBuf};

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        context! {
            title: "Learn one CWE every day",
            description: "Learn about one CWE every day, discover various types of vulnerabilities, and try to fix, reproduce, and study them",
            parent: "components/layout"
        },
    )
}

#[get("/static/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, files]) 
        .attach(Template::fairing())
}
