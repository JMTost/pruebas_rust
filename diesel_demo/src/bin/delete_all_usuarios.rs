extern crate diesel;
extern crate diesel_demo;

use self::diesel::prelude::*;
use self::diesel_demo::*;

fn main () {
    use diesel_demo::schema_usuarios::usuarios::dsl::*;

    let conexion = establecer_conexion();
    let elemento_borrado = diesel::delete(usuarios)
    .execute(&conexion)
    .expect("Error borrando el usuario");

    println!("Usuario eliminado, {}", elemento_borrado);
}