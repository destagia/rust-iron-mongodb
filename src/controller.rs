use types::*;

use std::path::Path;
use std::sync::Arc;

use iron::prelude::*;
use iron::status;
use iron::typemap::Key;

use mount::Mount;
use staticfile::Static;
use persistent::{Write, Read};

use router::Router;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use bson::Bson;


pub fn index(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

pub fn add_user(req: &mut Request) -> IronResult<Response> {
    let client = req.get::<Read<AppDB>>().unwrap();
    let ref params = req.extensions.get::<Router>().unwrap();
    let name = params.find("name").unwrap();
    let age = params.find("age").unwrap();

    let collection = client.db("iron").collection("users");
    let doc = doc! {"name" => name, "age" => age};

    collection.insert_one(doc.clone(), None).ok().expect("Failed!");

    Ok(Response::with((status::Ok, format!("{}, {}", name, age))))
}
