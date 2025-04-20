extern crate diesel;
extern crate diesel_demo;

use self::diesel::prelude::*;
use self::diesel_demo::*;
use std::env::args;

fn main() {
    use diesel_demo::schema_posts::posts::dsl::*;

    let id_entrada = args()
        .nth(1)
        .expect("No se encontro un identificador que sea igual al ingresado")
        .parse::<i32>().expect("ID invalido");

    let pattern = format!("%{}%", id_entrada);

    let conexion = establecer_conexion();
    //eliminiar registro mediante el id 
    let elemento_borrado = diesel::delete(posts.find(id_entrada))
        .execute(&conexion)
        .expect("ERROR BORRANDO LOS POSTS");
    /*
    let post = diesel::delete(posts.filter(title.like(pattern)))
        .execute(&conexion)
        .expect(&format!("Incapaz de encotnrar el post {}", id_entrada));

    */
    println!("Post eliminado, {}", elemento_borrado);
}
