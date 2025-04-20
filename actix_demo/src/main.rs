use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::http::header::ContentType;

use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Responder, ResponseError, delete, get, post, put,
    web,
};

use serde::{Deserialize, Serialize};

use std::fmt::Display;
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct Ticket {
    id: u32,
    author: String,
}

#[derive(Debug, Serialize)]
struct ErrNoId {
    id: u32,
    err: String,
}

struct AppState {
    tickets: Mutex<Vec<Ticket>>,
}

//implementación para el Responder para Ticket
impl Responder for Ticket {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let res_body = serde_json::to_string(&self).unwrap();

        //creamos el HttpResponse y se coloca el tipo de contenido
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(res_body)
    }
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

//CREACIÓN DE UN TICKET
#[post("/tickets")]
async fn post_ticket(req: web::Json<Ticket>, data: web::Data<AppState>) -> impl Responder {
    let new_ticket = Ticket {
        id: req.id,
        author: String::from(&req.author),
    };

    let mut tickets = data.tickets.lock().unwrap();

    let response = serde_json::to_string(&new_ticket).unwrap();

    tickets.push(new_ticket);
    HttpResponse::Created()
        .content_type(ContentType::json())
        .body(response)
}

//CONTROLADOR GET
//OBTENER TODOS LOS TICKETS
#[get("/tickets")]
async fn get_tickets(data: web::Data<AppState>) -> impl Responder {
    let tickets = data.tickets.lock().unwrap();

    let response = serde_json::to_string(&(*tickets)).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response)
}

//CONTROLADOR GET
//OBTENER EL TICKET MEDIANTE UN IDENTIFICADOR
#[get("/tickets/{id}")]
//obtenemos de la URL el identificador del ticket a buscar
async fn get_ticket(id: web::Path<u32>, data: web::Data<AppState>) -> Result<Ticket, ErrNoId> {
    println!("{id}");
    let ticket_id: u32 = *id;

    let tickets = data.tickets.lock().unwrap();

    let ticket: Vec<_> = tickets.iter().filter(|x| x.id == ticket_id).collect();

    if !ticket.is_empty() {
        Ok(Ticket {
            id: ticket[0].id,
            author: String::from(&ticket[0].author),
        })
    } else {
        let response = ErrNoId {
            id: ticket_id,
            err: String::from("Ticket not found"),
        };
        Err(response)
    }
}

//CONTROLADOR PUT
//actalización de ticket correspondiente a un id
#[put("/tickets/{id}")]
async fn update_ticket(id : web::Path<u32>, req : web::Json<Ticket>, data : web::Data<AppState>) -> Result<HttpResponse, ErrNoId> {

    let ticket_id = *id;
    
    let new_ticket = Ticket {
        id : ticket_id,
        author : String::from(&req.author)
    };

    let mut tickets = data.tickets.lock().unwrap();

    let id_index = tickets.iter()
    .position(|x| x.id == ticket_id);
    match id_index {
        Some(id) => {
            let response = serde_json::to_string(&new_ticket).unwrap();
            tickets[id] = new_ticket;
            Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
    .body(response))
        },
        None => {
            let response = ErrNoId {
                id : ticket_id,
                err : String::from("ticket no encontrado")
            };
            Err(response)
        }
    }
}


//CONTROLADOR DELETE
//eliminación de un ticket dependiendo del id correspondiente
#[delete("/tickets/{id}")]
async fn delete_ticket(id : web::Path<u32>, data : web::Data<AppState>) -> Result<Ticket, ErrNoId> {
    let ticket_id = *id;

    let mut tickets  = data.tickets.lock().unwrap();
    
    let id_index = tickets.iter().position(|x| x.id == ticket_id);

    match id_index {
        Some(id) => {
            let deleted_ticket = tickets.remove(id);
            let mensaje = serde_json::to_string("{
            'mensaje' : 'modificado con exito'
        }").unwrap();
            Ok(deleted_ticket)
        },
        None => {
            let response = ErrNoId {
                id : ticket_id,
                err : String::from("ticket no encontrado")
            };
            Err(response)
        }
        
    }
}

//CREACIÓN DEL SERVIDOR
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        tickets: Mutex::new(vec![
            Ticket {
                id: 1,
                author: String::from("Jane Doe"),
            },
            Ticket {
                id: 2,
                author: String::from("Patrick Star"),
            },
        ]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(post_ticket)
            .service(get_tickets)
            .service(get_ticket)
            .service(update_ticket)
            .service(delete_ticket)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
