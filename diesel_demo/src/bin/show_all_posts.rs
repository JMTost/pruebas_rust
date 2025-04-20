extern crate diesel;
extern crate diesel_demo;

use self::diesel::prelude::*;
use self::diesel_demo::*;
use self::models::*;

fn main () {
    use diesel_demo::schema_posts::posts::dsl::*;

    let conexion = establecer_conexion();
    let resultados = posts
    .limit(100)
    .load::<Post>(&conexion)
    .expect("Error cargando usuarios");

    println!("Mostrando los usuarios {}", resultados.len());

    for post in resultados {
        println!("{}", post.id);
        println!("----------");
        println!("{}", post.title);
        println!("----------");
        println!("{}", post.body);
        println!("----------");
        println!("{}", post.published);
        println!("*--------------------*");
    }


}