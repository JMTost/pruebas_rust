
use diesel::Queryable;
use serde::{Deserialize, Serialize};
use super::schema::usuarios;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Usuario {
    pub id : i32,
    pub nombre : String,
    pub email : String,
    pub edad : i32,
    pub tipousuario : String,
    pub activo : bool
}

#[derive(Insertable)]
#[table_name = "usuarios"]
pub struct NewUsuario<'a> {
    pub nombre : &'a str,
    pub email : &'a str,
    pub edad : i32,
    pub tipousuario : &'a str,
    pub activo : bool,
}