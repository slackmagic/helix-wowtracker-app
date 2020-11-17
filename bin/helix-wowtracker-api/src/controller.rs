use crate::state::AppState;
use crate::APP_NAME;
use actix_web::web::Data;
use actix_web::{web, HttpRequest, HttpResponse};
use helix_auth_lib::HelixAuth;

use std::sync::{Arc, Mutex};

pub fn healthcheck(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body(format!("Everything's fine on {}.", APP_NAME))
}

pub fn unimplemented(_req: HttpRequest) -> HttpResponse {
    HttpResponse::NotFound().body("unimplemented !")
}

pub fn version(_req: HttpRequest) -> HttpResponse {
    let version = helix_config_lib::version::Version::new(
        env!("CARGO_PKG_VERSION").to_owned(),
        env!("GIT_SHORT_HASH").to_owned(),
        env!("GIT_MESSAGE").to_owned(),
        env!("GIT_COMMIT_DATE").to_owned(),
    );

    HttpResponse::Ok().json(version)
}

//-----------------------------------------------------------------------------------

pub async fn get_all_characters_data(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    req: HttpRequest,
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();
    let claimer = HelixAuth::get_claimer(&req).unwrap();

    match domain.get_all_characters_data(&claimer.user_uuid).await {
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(characters) => HttpResponse::Ok().json(characters),
    }
}

pub async fn get_last_characters_data(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    req: HttpRequest,
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();
    let claimer = HelixAuth::get_claimer(&req).unwrap();

    match domain.get_last_characters_data(&claimer.user_uuid).await {
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(characters) => HttpResponse::Ok().json(characters),
    }
}

pub async fn test(wrap_state: Data<Arc<Mutex<AppState>>>, req: HttpRequest) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();
    let claimer = HelixAuth::get_claimer(&req).unwrap();

    let result = domain.update_all_characters_data(&claimer.user_uuid).await;
    match result {
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(_) => HttpResponse::Ok().body("Woooot"),
    }
}
