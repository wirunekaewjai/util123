pub mod functions;
pub mod routes;
pub mod structs;
pub mod views;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use functions::get_asset_map;
use structs::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let hostname = "0.0.0.0";
    let port = 8080;

    println!();
    println!("Running at http://0.0.0.0:{}", port);
    println!();

    HttpServer::new(move || {
        let asset_map = get_asset_map();
        let app_state = AppState { asset_map };

        App::new()
            .app_data(web::Data::new(app_state))
            .service(routes::get_util_page::handle)
            .service(routes::get_home_page::handle)
            .service(routes::get_asset::handle)
    })
    .bind((hostname, port))?
    .run()
    .await
}
