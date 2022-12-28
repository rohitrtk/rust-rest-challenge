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

    let localhost: String = String::from("127.0.0.1");
    let port: u16 = 8080;

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
            .configure(services::config)
    })
    .bind((localhost, port))?
    .run()
    .await
}
