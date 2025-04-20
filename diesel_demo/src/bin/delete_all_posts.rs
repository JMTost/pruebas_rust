extern crate diesel;
extern crate diesel_demo;

use self::diesel::prelude::*;
use self::diesel_demo::*;

fn main () {
    use diesel_demo::schema_posts::posts::dsl::*;

    let conexion = establecer_conexion();
    let elemento_borrado = diesel::delete(posts)
    .execute(&conexion)
    .expect("Error borrando el usuario");

    println!("Posts eliminados, {}", elemento_borrado);
}