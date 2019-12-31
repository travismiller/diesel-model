use crate::schema::posts;
use diesel_model_macros::model;

#[model(posts)]
#[derive(Identifiable, Queryable)]
#[table_name = "posts"] // Identifiable
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
