use actix_web::{App, HttpServer, web};

use server::routes::configure_routes;
use server::{AppConfig, AppState};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = AppConfig::load()?;
    let port = config.port;
    let app_state = AppState::new(&config.minapp);
    let _ = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .configure(configure_routes)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await;

    Ok(())
}
