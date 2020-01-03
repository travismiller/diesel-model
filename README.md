# Diesel Model

Model boilerplate for [Diesel](https://diesel.rs).

## Usage

Add a proc-macro attribute to your Diesel model specifying both the backend and
the schema module types.

Note that `Identifiable` and `Queryable` derives are currently required.

```rust
#[model(backend = diesel::mysql::Mysql, schema = crate::schema::posts)]
#[derive(Identifiable, Queryable)]
#[table_name = "posts"]
pub struct Post { /* … */ }
```

### Full Example

```rust
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_model_macros;

use diesel::prelude::*;

table! {
    posts (id) {
        id -> Integer,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

#[model(backend = Mysql, schema = posts)]
#[derive(Identifiable, Queryable)]
#[table_name = "posts"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

fn load_all_published_posts(
  connection: &MysqlConnection
) -> QueryResult<Vec<Post>> {
  Post::all()
      .filter(posts::dsl::published.eq(true))
      .load::<Post>(connection)
}
```

## License

Licensed under either of these:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)
