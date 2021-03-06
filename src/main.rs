//#![feature(proc_macro_hygiene)]

#![feature(decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
//************* with dynamic perameteres ****************//
//  #[get("/example/<value>")]
//  fn add(value: i32) -> String {
//      (value + 20).to_string()
//  }

//  fn main() {
//      rocket::ignite().mount("/", routes![add]).launch();
//  }

// ****************With qury parameters*************** //
// #[get("/example?cons=20&<value>")]
// fn add(value: i32) -> String {
//     (20 + value).to_string()
// }

// fn main() {
//     rocket::ignite().mount("/", routes![add]).launch();
// }

//*************with a little bit of frontend ************//
// use rocket_contrib::serve::StaticFiles;
// #[get("/example?cons=20&<value>")]
// fn add(value: i32) -> String {
//     (20 + value).to_string()
// }

// fn main() {

//     rocket::ignite()
//     .mount("/", StaticFiles::from("static"))
//     .mount("/", routes![add]).launch();
// }

use rocket_contrib::serve::StaticFiles;
#[get("/example?<value>")]
fn add(value: i32) -> String {
    (20 + value).to_string()
}

fn main() {
    rocket::ignite()
        .mount("/example", StaticFiles::from("static"))
        .mount("/", routes![add])
        .launch();
}
