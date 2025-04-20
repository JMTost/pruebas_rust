use diesel::Queryable;

use super::schema_posts::posts;
use super::schema_usuarios::usuarios;
 
#[derive(Queryable)]

pub struct Post{
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Queryable)]
pub struct Usuario{
    pub id : i32,
    pub nombre : String,
    pub email : String,
}


#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Insertable)]
#[table_name="usuarios"]
pub struct NewUsuario<'a>{
    pub nombre : &'a str,
    pub email : &'a str,
}