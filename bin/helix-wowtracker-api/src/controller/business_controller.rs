use crate::state::AppState;
use actix_web::web::Data;
use actix_web::{HttpRequest, HttpResponse};
use helix_auth_lib::HelixAuth;
use std::sync::{Arc, Mutex};

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

pub async fn get_previous_characters_data(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    req: HttpRequest,
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();
    let claimer = HelixAuth::get_claimer(&req).unwrap();

    match domain
        .get_previous_characters_data(&claimer.user_uuid)
        .await
    {
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(characters) => HttpResponse::Ok().json(characters),
    }
}

pub async fn update_characters(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    req: HttpRequest,
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();
    let claimer = HelixAuth::get_claimer(&req).unwrap();

    let result = domain.update_all_characters_data(&claimer.user_uuid).await;
    match result {
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(_) => HttpResponse::Ok().body("Characters udpated."),
    }
}
