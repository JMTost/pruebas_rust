
# 📘 API de Gestión de Usuarios

  

API RESTful desarrollada con **Rust**, **Actix Web** y **Diesel**. Permite registrar, consultar, actualizar y activar usuarios almacenados en una base de datos PostgreSQL.

  

---

  

## 🔧 Requisitos

  

- Rust

- PostgreSQL

- Diesel CLI

Instalar con:

```

cargo install diesel_cli --no-default-features --features postgres
```
Archivo .env con la variable DATABASE_URL.

Ejemplo:
```
DATABASE_URL=postgres://usuario:password@localhost/mi_base
```
----
# 🚀 Ejecución  

Ejecuta las migraciones con Diesel (si ya están definidas):

Corre la API:
```
cargo run
```
  

La API estará disponible en:

http://127.0.0.1:3000

---  

# 📚 Endpoints Disponibles

 1.  Crear Usuario

URL: /creaUsuario
Método: POST
Body (JSON):
{

"id": 0,
"nombre": "nombre",
"email": "nombre@example.com",
"edad": 25,
"tipousuario": "Capturista",
"activo": false
}
  Respuesta: Objeto JSON del usuario creado.

 2. Obtener Todos los Usuarios
URL: /obtenUsuarios
Método: GET
Respuesta (ejemplo):
[
{
"activo": true,
"edad": 30,
"email": "jesust@example.com",
"nombre": "Jesus Toledo",
"tipousuario": "Administrador"
},
{
"activo": true,
"edad": 25,
"email": "nombre@example.com",
"nombre": "nombre",
"tipousuario": "Capturista"
}
]
 3. Obtener Usuario por ID
URL: /obtenUsuario/{id}
Método: GET
Respuesta (ejemplo):
{
"activo": false,
"edad": 24,
"email": "jesust@example.com",
"nombre": "Jesus Toledo",
"tipousuario": "Administrador"
}
4. Actualizar Usuario
URL: /actualizaUsuario/{id}
Método: PUT
Body (JSON):
{
"id": 0,
"nombre": "Jesus Toledo",
"email": "jesust@example.com",
"edad": 30,
"tipousuario": "Administrador",
"activo": true
}
Respuesta (ejemplo):
{
"mensaje": "Se modifico el usuario de forma exitosa",
"dato": 1
}
5. Activar Usuario
URL: /actualizaEstadoUsuario/{id}
Método: PUT
Descripción: Cambia el campo activo a true.
Respuesta (ejemplo):
{
"mensaje": "Se actualizo de forma exitosa el estado del usuario",
"id": 2,
"dato": 1
}
----
  

🧪 Pruebas con Postman

  

Puedes usar los ejemplos anteriores para probar cada endpoint.

