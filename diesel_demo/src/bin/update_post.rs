extern crate diesel;
extern crate diesel_demo;



use self::diesel::prelude::*;
use self::diesel_demo::*;
use std::env::args;
use std::io::stdin;

fn main() {
    use diesel_demo::schema_posts::posts::dsl::{posts, title, body};
    let conexion = establecer_conexion();
    
    let id_entrada = args()
        .nth(1)
        .expect("No e encontro un identificador que sea igual al ingresado")
        .parse::<i32>()
        .expect("ID invalido");
    let mut titulo = String::new();
    let mut body_entrada = String::new();

    println!("Ingresa el titulo que desees utilizar: ");
    stdin().read_line(&mut titulo).unwrap();
    let titulo = &titulo[..(titulo.len() - 1)];
    println!("\nAHORA PARA INGRESA EL MENSAJE DE {} (PRESIONA {} CUANDO TERMINES)\n", titulo, EOF);
    stdin().read_line(&mut body_entrada).unwrap();

    let result = diesel::update(posts.find(id_entrada))
    .set((title.eq(titulo), body.eq(body_entrada)))
    .execute(&conexion);

    match result {
        Ok(affected_rows) => {
            println!("Se actualizaron {} fila(s)", affected_rows);
        }
        Err(e)=>{
            eprintln!("Error al actualizar el post: {:?}", e);
        }
        
    }

}
#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";