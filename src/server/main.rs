pub mod functions;
pub mod routes;
pub mod structs;
pub mod views;

use actix_web::{web, App, HttpServer};
use chrono::Utc;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let hostname = "0.0.0.0";
    let port = 8080;

    println!();
    println!("Running at http://{hostname}:{port}");
    println!();

    let version = Utc::now().timestamp_millis().to_string();
    let asset_map = functions::get_asset_map();

    HttpServer::new(move || {
        let app_state = structs::AppState {
            asset_map: asset_map.clone(),
        };

        let app = App::new()
            .app_data(web::Data::new(app_state))
            .service(routes::util::handle)
            .service(routes::home::handle)
            .service(routes::asset::handle)
            .service(routes::public::handle);

        if cfg!(debug_assertions) {
            app.app_data(web::Data::new(version.clone()))
                .service(routes::api::version::handle)
        } else {
            app
        }
    })
    .bind((hostname, port))?
    .run()
    .await
}
