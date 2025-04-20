extern crate diesel;
extern crate diesel_demo;
//DEFINE QUE SE EMPLEARA LAS FUNCION
use self::diesel::prelude::*;
use self::diesel_demo::*;
use self::models::*;

fn main() {
    use diesel_demo::schema_posts::posts::dsl::*;

    let conexion = establecer_conexion();
    let resultados = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&conexion)
        .expect("Error cargando posts");

    println!("Mostrando posts {}", resultados.len());

    for post in resultados {
        println!("{}", post.title);
        println!("-----------------\n");
        println!("{}", post.body);
    }

}
