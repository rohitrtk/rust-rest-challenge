use actix_web::{get, web, App, HttpServer};
use std::sync::Mutex;

mod actorlist;
use actorlist::services;
use actorlist::models::ActorListEntry;

struct AppState {
    actorlist_entries: Mutex<Vec<ActorListEntry>>
}

#[get("/")]
async fn index() -> String {
    "Health Check".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        actorlist_entries: Mutex::new(vec![])
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
            .configure(services::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
