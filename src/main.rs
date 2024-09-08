//this enables procedural macros that allow the creation of endpoints using decorator functions like 'get()' below,
//which turns the fn index() that it precedes into an endpoint
#![feature(proc_macro_hygiene, decl_macro)]


//'macro_use' here is specifying that all the macros and decorator functions in Rocket should be imported into the project
//this is how we get access to 'get()' in the decorator below here
#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!  Because I ran 'cargo watch -x run', it makes it reload when I press save to any of the files on here (that aren't in .gitignore)"
}

// #[get("/about")]
// fn about() -> &'static str{
//     "<h1>About Page</h1>"
// }

#[launch]
fn launch_rocket() -> _ {

        //create sqlite database
        let db_connection = rusqlite::Connection::open("data.squlite");

        // rocket::ignite().mount("/", routes![index].launch());
        rocket::build().mount("/", routes![index])
    
}


