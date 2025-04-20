extern crate diesel;
extern crate diesel_demo;

use self::diesel::prelude::*;
use self::diesel_demo::*;
use self::models::*;

fn main () {
    use diesel_demo::schema_usuarios::usuarios::dsl::*;

    let conexion = establecer_conexion();
    let resultados = usuarios
    .limit(100)
    .load::<Usuario>(&conexion)
    .expect("Error cargando usuarios");

    println!("Mostrando los usuarios {}", resultados.len());

    for usuario in resultados {
        println!("{}", usuario.nombre);
        println!("----------");
        println!("{}", usuario.email);
        println!("*--------------------*");
    }


}