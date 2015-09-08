use iron::typemap::Key;
use mongodb::Client;
use bson::Bson;

pub struct AppDB;
impl Key for AppDB { type Value = Client; }

pub struct User {
    name: String,
    age: i32
}
