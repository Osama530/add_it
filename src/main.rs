 	//#![feature(proc_macro_hygiene)]
	
     #![feature(decl_macro)]
	
     #[macro_use]
     extern crate rocket;
     
     #[get("/example/<value>")]
     fn add(value: i32) -> String {
         (value + 20).to_string()
     }
     
     fn main() {
         rocket::ignite().mount("/", routes![add]).launch();
     }