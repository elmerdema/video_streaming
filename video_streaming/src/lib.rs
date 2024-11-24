use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Mutex, Arc};

//make request the post endpoint
// curl -X POST "http://localhost:8080/users" -H "Content-Type: application/json" -d "{\"name\": \"John Doe\"}"


#[derive(Serialize,Deserialize)]
struct User {
    name: String,
}

#[derive(Serialize,Deserialize)]
struct CreateUserResponse {
    id: u32,
    name: String,
}

type UserDb = Arc<Mutex<HashMap<u32,User>>>;

#[actix_web::get("/greet/{id}")]
async fn greet(user_id: web::Path<i32>) -> impl Responder {
    format!("Hello world {}", user_id)
}

#[actix_web::post("/users")]
async fn create_user(
    user_data: web::Json<User>,
    db: web::Data<UserDb>,
) -> impl Responder {
    let mut db = db.lock().unwrap();
    let new_id= db.keys().max().unwrap_or(&0) +1;
    let name = user_data.name.clone();
    db.insert(new_id, user_data.into_inner());
    HttpResponse::Created().json( CreateUserResponse{
        id: new_id,
        name,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("Server started on port {port}");

    let user_db: UserDb = Arc::new(Mutex::new(HashMap::<u32,User>::new()));

    HttpServer::new(move || {
        let app_data = web::Data::new(user_db.clone());
        App::new()
            .app_data(app_data)
            .service(greet)
            .service(create_user)
    })
    .bind(("127.0.0.1", port))?
    .workers(2)
    .run()
    .await
}
