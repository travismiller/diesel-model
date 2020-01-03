use crate::schema::posts;
use diesel::mysql::Mysql;

#[model(backend = Mysql, schema = posts)]
#[derive(Identifiable, Queryable)]
#[table_name = "posts"] // Identifiable
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
