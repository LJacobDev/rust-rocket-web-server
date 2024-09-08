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
    println!("Starting server");
    println!("Initializing database");

    //create sqlite database
    let db_connection = rusqlite::Connection::open("data.sqlite")
        .expect("Database connection failed, terminating program");

    //this method takes in a sql string and a params collection
    //because no params are needed in this sql statement, the params argument can be given () or [].
    //It used to be possible to use rusqlite::NO_PARAMS but that has since been removed
    //and &[] isn't allowed in this example
    let connection_result = db_connection.execute(
        "create table if not exists test (id integer primary key);",
        //&[],                  //not permitted
        //rusqlite::NO_PARAMS   //has been removed from rusqlite
        // [],                  //permitted
        (),                     //also permitted
    );

    println!("Connection result: {:?}", connection_result);

    // rocket::ignite().mount("/", routes![index].launch());
    rocket::build().mount("/", routes![index])
}
