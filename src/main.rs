use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use std::path::PathBuf;


#[derive(Serialize, Deserialize, Clone)]
struct Video {
    id: String,
    title: String,
    description: String,
    filename: String,
}

type Db= Mutex<HashMap<String, Video>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut videos = HashMap::new();
    videos.insert(
        "1".to_string(),
        Video {
        id: "1".to_string(),
        title: "Sample Video".to_string(),
        description: "An example video.".to_string(),
        filename: "example.mp4".to_string(),
        }
    );

    let db = web::Data::new(Db::new(videos));

    HttpServer::new(move|| {
        App::new()
        .app_data(db.clone())
        .route("/videos/{id}/metadata", web::get().to(get_video_metadata))
        .route("/videos/{filename}", web::get().to(stream_video))

    })
    .bind("127.0.0.1:8080")?
    .await;
}

async fn get_video_metadata(
    db: web::Data<Db>,
    web::Path(id): web::Path<String>,
) -> impl Responder {
    let db = db.lock().unwrap();
    if let Some(video) = db.get(&id) {
        HttpResponse::Ok().json(video)
    } else {
        HttpResponse::NotFound().body("Video not found")
    }
}

async fn stream_video(web::Path(filename): web::Path<String>) -> impl Responder {
    let path = PathBuf::from(format!("videos/{}",filename));
    if path.exists() {
        actix_files::NamedFile::open(path)
        .map(|file| file.into_response())
        .unwrap_or_else(|_| HttpResponse::InternalServerError())

    } else {
        HttpResponse::NotFound().body("File not found")
    }
}