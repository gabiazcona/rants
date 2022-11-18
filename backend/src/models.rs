use diesel::prelude::*;
use serde::Serialize;
use crate::schema::rants;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Queryable, Serialize)]
pub struct Rant {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub username: String,
    //pub dateposted: String,
}

#[derive(Insertable)]
#[diesel(table_name = rants)]
pub struct NewRant<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub username: &'a str,
}