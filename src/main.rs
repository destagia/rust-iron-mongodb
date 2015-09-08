extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate persistent;
#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

mod controller;
mod types;

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


fn main() {
    let mut router = Router::new();
    let client: Client = Client::connect("localhost", 27017).ok().expect("Failed!");

    router.get("/", controller::index);
    router.get("/users/:name/:age", controller::add_user);

    let mut mount = Mount::new();
    mount.mount("/", router);
    mount.mount("/static", Static::new(Path::new("./src/static/")));

    let mut middleware = Chain::new(mount);
    middleware.link(Read::<AppDB>::both(client));

    Iron::new(middleware).http("localhost:3000").unwrap();

}
