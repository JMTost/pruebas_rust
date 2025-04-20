extern crate diesel;
extern crate diesel_demo;

use self::diesel::prelude::*;
use self::diesel_demo::*;
use self::models::Post;
use std::env::args;

fn main() {
    use diesel_demo::schema_posts::posts::dsl::{posts, published};
    //{posts, published}

    let id = args()
        .nth(1)
        .expect("las publicaciones de post requere un identificador de post")
        .parse::<i32>().expect("ID invalido");
    let conexion = establecer_conexion();

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(&conexion)
        .expect(&format!("Incapaz de encontrar el post {}", id));
    println!("Post publicado {}", post.title);
}
