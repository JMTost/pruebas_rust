pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::web::Json;
use diesel::result;
use diesel::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::Pool;
use serde_json::Value;
use serde_json::json;

use self::models::{NewUsuario, Usuario};

use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::http::header::ContentType;

use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Responder, ResponseError, delete, get, post, put,
    web,
};
use api_prueba::crear_usuario;

use api_prueba::schema::usuarios::dsl::*;
use api_prueba::schema::usuarios::*;
use diesel::r2d2::{self, ConnectionManager};

use serde::{Deserialize, Serialize};

use std::fmt::Display;
use std::sync::Mutex;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Serialize)]
struct ErrNoId {
    id: u32,
    err: String,
}

impl ResponseError for ErrNoId {
    fn status_code(&self) -> StatusCode {
        StatusCode::NOT_FOUND
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let body = serde_json::to_string(&self).unwrap();
        let res = HttpResponse::new(self.status_code());
        res.set_body(BoxBody::new(body))
    }
}

impl Display for ErrNoId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

//Creación de un usuario
#[post("/creaUsuario")]
async fn post_user(pool: web::Data<DbPool>, req: web::Json<Usuario>) -> impl Responder {
    let conexion = pool.get().expect("Error obteniendo conexión del pool");

    let usuario_nuevo = crear_usuario(
        &conexion,
        &req.nombre,
        &req.email,
        req.edad,
        &req.tipousuario,
        req.activo,
    );
    HttpResponse::Ok().json(usuario_nuevo)
}

#[get("/obtenUsuarios")]
async fn get_users(pool: web::Data<DbPool>) -> impl Responder {
    let conexion = pool.get().expect("ERROR OBTENIENDO CONEION DEL POOL");
    let resultados = usuarios
        .load::<Usuario>(&conexion)
        .expect("ERROR AL OBTENER LOS USUARIOS");
    let mut objeto_usuarios: Vec<Value> = Vec::new();
    for usuario in resultados {
        let elemento = json!({
            "nombre" : usuario.nombre,
            "email" : usuario.email,
            "edad" : usuario.edad,
            "tipousuario" : usuario.tipousuario,
            "activo" : usuario.activo,
        });
        objeto_usuarios.push(elemento);
    }

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&objeto_usuarios).unwrap())
}

//CONTROLADOR GET / OTBENER UN USUARIO
#[get("/obtenUsuario/{id}")]
async fn get_user(path: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    let conexion = pool.get().expect("ERROR OBTENIENDO CONEION DEL POOL");
    let id_entrada = *path;

    let usuario_obtenido = usuarios
        .filter(id.eq(id_entrada))
        .limit(1)
        .load::<Usuario>(&conexion)
        .expect("Error cargando el usuario");

        let mut objeto_usuarios: Vec<Value> = Vec::new();

    if !usuario_obtenido.is_empty() {
        for elemento in usuario_obtenido {
            let result = json!({
                "nombre" : elemento.nombre,
                "email" : elemento.email,
                "edad" : elemento.edad,
                "tipousuario" : elemento.tipousuario,
                "activo" : elemento.activo,
            });
            objeto_usuarios.push(result);
        }
        HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&objeto_usuarios[0]).unwrap())
    } else {
       let response = ErrNoId {
        id : id_entrada as u32,
        err : String::from("EL usuario no se encontro"),
       };
       HttpResponse::BadRequest()
       .content_type(ContentType::json())
       .body(serde_json::to_string(&response).unwrap())
    }
    
}

//CONTROLADOR PUT / ACTUALIZAR USUARIO
#[put("/actualizaUsuario/{id}")]
async fn update_user(path : web::Path<i32>, req : web::Json<Usuario>, pool : web::Data<DbPool>) -> impl Responder {
    let conexion = pool.get().expect("ERROR OBTENIENDO CONEION DEL POOL");
    let id_usuario = *path;
    //println!("{id_usuario}");
    let resultado = diesel::update(usuarios.find(id_usuario))
    .set((nombre.eq(&req.nombre), email.eq(&req.email), edad.eq(req.edad), tipousuario.eq(&req.tipousuario), activo.eq(req.activo)))
    .execute(&conexion);

    match resultado {
        Ok(affected_rows) => {
            let response = json!({
                "mensaje" : "Se modifico el usuario de forma exitosa",
                "dato" : affected_rows
            });
            HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&response).unwrap())
        }
        Err(e) => {
            let response = json!({
                "mensaje" : "No se encontro el usuario",
                "id" : id_usuario,
            });
            HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&response).unwrap())
        }
    }

}


//CONTROLADOR PUT / ACTUALIZA ESTADO
#[put("/actualizaEstadoUsuario/{id}")]
async fn update_status_user(path : web::Path<i32>, pool : web::Data<DbPool>) -> impl Responder {
    let conexion = pool.get().expect("ERROR OBTENIENDO CONEION DEL POOL");
    let id_entrada = *path;
    let resultado = diesel::update(usuarios.find(id_entrada))
    .set((activo.eq(true)))
    .execute(&conexion);

    match resultado {
        Ok(affected_rows) => {
            let response = json!({
                "mensaje" : "Se actualizo de forma exitosa el estado del usuario",
                "id" : id_entrada,
                "dato" : affected_rows
            });
            HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&response).unwrap())
        }
        Err(e) => {
            let response = json!({
                "mensaje" : "No se encontro el usuario",
                "id" : id_entrada,
            });
            HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&response).unwrap())
        }
    }

}
// async fn get_user(pool: web::Data<DbPool>, id: web::Path<u32>) -> impl Responder {

//     let user_id = *id;

//     HttpResponse::Ok()

//     // use api_prueba::schema::usuarios::dsl::*;
//     // use api_prueba::schema::usuarios::*;
//     // println!("{id}");
//     // let user_id : u32= *id;
//     // let conexion = pool.get().expect("ERROR OBTENIENDO CONEION DEL POOL");

//     // let usuario_obtenido = usuarios
//     //     .filter(id.eq(user_id))
//     //     .limit(1)
//     //     .load::<Usuario>(&conexion)
//     //     .expect("Error cargando el usuario");

//     // let mut objeto_usuarios: Vec<Value> = Vec::new();

//     // if !usuario_obtenido.is_empty() {
//     //     Ok(Usuario {
//     //         id: usuario_obtenido[0].id,
//     //         nombre: usuario_obtenido[0].nombre,
//     //         email: usuario_obtenido[0].email,
//     //         edad: usuario_obtenido[0].edad,
//     //         tipousuario: usuario_obtenido[0].tipousuario,
//     //         activo: usuario_obtenido[0].activo,
//     //     })
//     // } else {
//     //     let response = ErrNoId {
//     //         id: user_id,
//     //         err: String::from("EL usuario no se encontro"),
//     //     };
//     //     Err(response)
//     // }

// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let url_bd =
        std::env::var("DATABASE_URL").expect("LA URL DE LA BASE DE DATOS DEBE SER DEFINIDA");
    let manager = ConnectionManager::<PgConnection>::new(url_bd);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("No se pudo construir el pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(post_user)
            .service(get_users)
            .service(get_user)
            .service(update_user)
            .service(update_status_user)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
    // dotenv::dotenv().ok();
    // let url_db: String = env::var("DATABASE_URL").expect("LA URL DE LA BASE DE DATOS DEBE SER DEFINIDA");
    // let manager = ConnectionManager::<PgConnection>::new(database_url);
    // let pool = r2d2::Pool::builder()
    // .build(manager)
    // .expect("No se pudo construir el pool");

    // HttpServer::new(move || {
    //     App::new()
    //     .app_data(post_user)
    // })
    // .bind(("127.0.0.1", 3000))?
    // .run()
    // .await
}
