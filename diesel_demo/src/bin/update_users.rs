extern crate diesel;
extern crate diesel_demo;

use self::diesel::prelude::*;
use self::diesel_demo::*;
use std::env::args;
use std::io::stdin;

fn main () {
    use diesel_demo::schema_usuarios::usuarios::dsl::{usuarios, nombre, email};

    let conexion = establecer_conexion();

    let id_entrada = args()
    .nth(1)
    .expect("No se encontro un identificador que sea igual al ingresado")
    .parse::<i32>()
    .expect("ID invalido");

    let mut nombre_user = String::new();
    let mut email_user = String::new();

    println!("Ingresa el nombre que deseas utilizar: ");
    stdin().read_line(&mut nombre_user).unwrap();
    let nombre_usuario = nombre_user.trim();

    println!("\nAHORA inresa el email que deseas utilizar: ");
    stdin().read_line(&mut email_user).unwrap();
    let email_usuario = email_user.trim();

    let result = diesel::update(usuarios.find(id_entrada))
    .set((nombre.eq(nombre_usuario), email.eq(email_usuario)))
    .execute(&conexion);

    match result {
        Ok(affected_rows) => {
            println!("Se actualizaron {} fila(s)", affected_rows);
        }
        Err(e) => {
            eprintln!("Error al actualizar el usuario: {:?}", e);
        }
    }


}