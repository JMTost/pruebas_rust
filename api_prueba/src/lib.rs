pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;

use std::env;

use self::models::{Usuario, NewUsuario};


pub fn crear_usuario<'a>(conexion : &PgConnection, nombre : &'a str, email : &'a str, edad : i32, tipo_usuario : &'a str, activo : bool) -> Usuario {
    use schema::usuarios;

    let new_usuario = NewUsuario {
        nombre : nombre,
        email : email,
        edad : edad,
        tipousuario : tipo_usuario,
        activo : activo
    };

    diesel::insert_into(usuarios::table)
    .values(&new_usuario)
    .get_result(conexion)
    .expect("Error guardando el nuevo usuario")

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
