#[macro_use]
extern crate serde_derive;

pub mod configuration;
pub mod controller;
pub mod state;

use crate::controller::{business_controller::*, internal_controller::*};
use crate::state::AppState;
use actix_web::{middleware, web, App, HttpServer};
use helix_auth_lib::middleware::AuthValidator;
use helix_config_lib::Configuration as GlobalConfiguration;
use std::sync::{Arc, Mutex};
use std::{env, io};

const APP_NAME: &str = "WOWTRACKER_APP";

#[actix_web::main]
async fn main() -> io::Result<()> {
    println!("[HELIX {} {}]", APP_NAME, env!("CARGO_PKG_VERSION"));

    //Configuration init.
    let configuration = GlobalConfiguration::new();

    //Set the IP:PORT to be served.
    let addr = configuration.get_served_addr();
    print!("--> Started on ");
    println!("http://{}", addr);

    //Logger service initialization
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    //Define a global state for all the Actix-Worker
    let app_state = Arc::new(Mutex::new(AppState::new()));

    //Start server
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(AuthValidator::new(get_exception_uri()))
            .data(app_state.clone())
            .service(
                web::scope("/api")
                    .route("/_", web::get().to(healthcheck))
                    .service(web::scope("/wow-tracker").configure(get_routes_configuration)),
            )
            .service(web::scope("").route("/{filename:.*}", web::get().to(serve_static_file)))
    })
    .bind(&addr)
    .expect("Cannot bind to address.")
    .keep_alive(configuration.get_keep_alive())
    .shutdown_timeout(configuration.get_shutdown_time_out())
    .workers(configuration.get_workers_number())
    .run()
    .await
}

fn get_routes_configuration(cfg: &mut web::ServiceConfig) {
    //----------------------------------------------------------
    //___DOMAIN___
    //----------------------------------------------------------

    cfg.service(
        web::scope("").service(
            web::scope("/characters")
                .route("", web::get().to(get_all_characters_data))
                .route("/last", web::get().to(get_last_characters_data))
                .route("/previous", web::get().to(get_previous_characters_data))
                .route("/update", web::get().to(update_characters)),
        ),
    );
}

fn get_exception_uri() -> Vec<String> {
    let mut exception_uri = Vec::new();
    exception_uri.push("/api/_".to_string());
    exception_uri.push("/api/version".to_string());
    exception_uri.push("/api/login".to_string());
    exception_uri
}
