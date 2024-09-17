//this enables procedural macros that allow the creation of endpoints using decorator functions like 'get()' below,
//which turns the fn index() that it precedes into an endpoint
#![feature(proc_macro_hygiene, decl_macro)]

//'macro_use' here is specifying that all the macros and decorator functions in Rocket should be imported into the project
//this is how we get access to 'get()' in the decorator below here
#[macro_use]
extern crate rocket;
use rocket::{http::Status, serde::{json::Json, Deserialize, Serialize}};

//Data structures for the to-do list functions

#[derive(Serialize)]
struct TodoList {
    items: Vec<TodoItem>,
}

#[derive(Serialize)]
struct TodoItem {
    id: i64,
    item: String,
}

#[derive(Serialize)]
struct StatusMessage {
    message: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!  Because I ran 'cargo watch -x run', it makes it reload when I press save to any of the files on here (that aren't in .gitignore)"
}

#[post("/todo", format = "json", data = "<item>")]
fn add_todo_item(item: Json<String>) -> Result<Json<StatusMessage>, String> {

    let db_connection = match rusqlite::Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => return Err("Unable to connect to database".to_string())
    };

    let mut statement = match db_connection.prepare("insert into todo_list (id, item) values (null, $1);") {
        Ok(statement) => statement,
        Err(_) => return Err("Unable to prepare query".to_string())
    };

    //item.0 is referring to 'item' the input parameter, which is a json object that is being given via POST
    //it has a .0 value but it wasn't explained much here how that exactly works - it looks like accessing part of a tuple here
    let results = statement.execute(&[&item.0]);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage { message: format!("Updated {} rows", rows_affected)})),
        Err(_) => return Err("Unable to execute query".to_string())
    }
}


#[get("/todo")]
fn fetch_all_todo_items() -> Result<Json<TodoList>, String> {
    let db_connection = match rusqlite::Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err("Failed to connect to database".to_string());
        }
    };

    let mut statement = match db_connection.prepare("SELECT id, item FROM todo_list;") {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".to_string()),
    };

    let results = statement.query_map((), |row| {
        Ok(TodoItem {
            id: row.get(0)?,
            item: row.get(1)?,
        })
    });

    match results {
        Ok(rows) => {
            let collection: rusqlite::Result<Vec<TodoItem>> = rows.collect();

            match collection {
                Ok(items) => Ok(Json(TodoList { items })),
                Err(_) => Err("Could not get items".to_string()),
            }
        }
        Err(_) => Err("Failed to fetch todo items".to_string()),
    }

    //using .into() here allows the compiler to infer that it needs to turn &str into String,
    //due to the fact that the return value is a Result with a String type for its error outcome
    // Err("Unknown Error".into())
}

// #[get("/about")]
// fn about() -> &'static str{
//     "<h1>About Page</h1>"
// }

#[launch]
fn launch_rocket() -> _ {
    println!("Starting server");
    println!("Initializing database");

    {
        //wrapper scope created for db_connection - see ending brace for comments

        //create sqlite database
        let db_connection = rusqlite::Connection::open("data.sqlite")
            .expect("Database connection failed, terminating program");

        //this method takes in a sql string and a params collection
        //because no params are needed in this sql statement, the params argument can be given () or [].
        //It used to be possible to use rusqlite::NO_PARAMS but that has since been removed
        //and &[] isn't allowed in this example
        let table_result = db_connection.execute(
            "create table if not exists todo_list (
            id integer primary key,
            item varchar(64) not null
            );",
            //&[],                  //not permitted
            //rusqlite::NO_PARAMS   //has been removed from rusqlite
            // [],                  //permitted
            (), //also permitted
        );

        println!("Table result: {:?}", table_result);
    } //wrapper scope is intended to drop db_connection at this point,
      //because rocket is multi threaded, and sqlite is not multi threaded
      //so in this case a new connection is going to be opened for each time an endpoint is accessed
      //this is not what would normally be done in a proper database, but it will work for this simple example application

    // rocket::ignite().mount("/", routes![index].launch());
    rocket::build().mount("/", routes![index, fetch_all_todo_items])
}
