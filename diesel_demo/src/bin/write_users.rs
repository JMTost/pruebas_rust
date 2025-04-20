extern crate diesel_demo;
extern crate diesel;

use self::diesel_demo::*;
use std::io::{stdin, Read};

fn main() {
    let conexion = establecer_conexion();

    println!("Ingresa el nombre del usuario: ");
    let mut nombre = String::new();
    stdin().read_line(&mut nombre).unwrap();
    let nombre = nombre.trim();
    //let nombre = &nombre[..(nombre.len() -1)];

    println!("\n OK! ahora ingresa el email del usuario {} (Cuando termines preciona {})\n", nombre, EOF);
    let mut email = String::new();
    stdin().read_to_string(&mut email).unwrap();
    
    let usuario = crear_usuario(&conexion, nombre, &email);
    println!("\nSe almaceno {} con el id {}", nombre, usuario.id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

