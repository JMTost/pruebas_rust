pub mod models;
pub mod schema_posts;
pub mod schema_usuarios;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;

use std::env;

use self::models::{NewPost, Post};
use self::models::{NewUsuario, Usuario};

pub fn establecer_conexion() -> PgConnection {
    dotenv().ok();

    let url_basededatos =
        env::var("DATABASE_URL").expect("LA URL DE LA BASE DE DATOS DEBE SET DEFINIDA");
    PgConnection::establish(&url_basededatos)
        .expect(&format!("Error conectando a {}", url_basededatos))
}

pub fn crear_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
    use schema_posts::posts;

    let new_post = NewPost {
        title: title,
        body: body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error guardando un nuevo post")
}


pub fn crear_usuario<'a>(conexion: &PgConnection, nombre: &'a str, email: &'a str) -> Usuario {
    use schema_usuarios::usuarios;

    let new_usuario = NewUsuario { 
        nombre: nombre, 
        email: email,
    };

    diesel::insert_into(usuarios::table) 
    .values(&new_usuario)
    .get_result(conexion)
    .expect("Error guardando al nuevo usuario")
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
