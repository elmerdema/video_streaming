use actix_files::NamedFile;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
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

type Db = Mutex<HashMap<String, Video>>;

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
        },
    );

    let db = web::Data::new(Db::new(videos));

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .route("/videos/{id}/metadata", web::get().to(get_video_metadata))
            .route("/videos/{filename}", web::get().to(stream_video))
    })
    .bind("0.0.0.0:8080")? 
    //http://your_local_ip:8080/videos/1/metadata
    //http://your_local_ip:8080/videos/example.mp4
    //.bind("127.0.0.1:8080")? for local testing, use ipconfig to 
    .run()
    .await
}

async fn get_video_metadata(
    db: web::Data<Db>,
    id: web::Path<String>,
) -> impl Responder {
    let db = db.lock().unwrap();
    if let Some(video) = db.get(&id.into_inner()) {
        HttpResponse::Ok().json(video)
    } else {
        HttpResponse::NotFound().body("Video not found")
    }
}

async fn stream_video(
    filename: web::Path<String>
) -> actix_web::Result<NamedFile> {
    let path = PathBuf::from(format!("videos/{}", filename.into_inner()));
    if path.exists() {
        Ok(NamedFile::open(path)?)
    } else {
        Err(actix_web::error::ErrorNotFound("File not found"))
    }
}