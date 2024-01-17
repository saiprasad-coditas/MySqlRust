use actix_web::{web, App, HttpServer};
use std::sync::{Arc, Mutex};

mod api;
mod database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(api::AppState {
        managers: Arc::new(Mutex::new(Vec::new())),
        employees: Arc::new(Mutex::new(Vec::new())),
    });

    // Create and initialize the database
    let database = database::Database::new();
    if let Err(err) = database.create_tables() {
        eprintln!("Failed to create tables: {}", err);
        return Ok(());
    }

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(api::configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
