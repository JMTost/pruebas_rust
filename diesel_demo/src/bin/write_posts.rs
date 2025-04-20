extern crate diesel_demo;
extern crate diesel;

use self::diesel_demo::*;
use std::io::{stdin, Read};

fn main () {
    let conexion = establecer_conexion();
    
    println!("Ingresa el titulo: ");
    let mut titulo = String::new();
    stdin().read_line(&mut titulo).unwrap();
    //hace un trim para quitar el elemento de nueva linea
    let titulo = &titulo[..(titulo.len() -1)];
    println!("\n OK! Let's write {} (Press {} when finished)\n", titulo, EOF);
    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    let post = crear_post(&conexion, titulo, &body);
    println!("\nSAVED draft {} with id {}", titulo, post.id)
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";