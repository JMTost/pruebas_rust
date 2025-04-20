extern crate diesel;
extern crate diesel_demo;

use self::diesel::prelude::*;
use self::diesel_demo::*;
use std::env::args;

fn main () {
    use diesel_demo::schema_usuarios::usuarios::dsl::*;

    let id_usuario = args()
    .nth(1)
    .expect("No es un identificador aceptado.")
    .parse::<i32>().expect("ID invalido");

    let conexion = establecer_conexion();
    let elemento_borrado = diesel::delete(usuarios.find(id_usuario))
    .execute(&conexion)
    .expect("Error borrando el usuario");

    println!("Usuario eliminado, {}", elemento_borrado);
}