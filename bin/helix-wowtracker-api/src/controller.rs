use crate::configuration::Configuration;
use crate::state::AppState;
use crate::APP_NAME;
use actix_files::NamedFile;
use actix_web::web::Data;
use actix_web::{web, HttpRequest, HttpResponse, Result};
use helix_auth_lib::HelixAuth;
use std::path::PathBuf;

use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize)]
struct HealthCheckResponse {
    app_name: String,
    message: String,
}

pub async fn serve_static_file(req: HttpRequest) -> Result<NamedFile> {
    let filename: &str = req.match_info().query("filename");
    let base_path = Configuration::get_static_folder();

    let filename = match filename.contains(".") {
        true => filename,
        false => "index.html",
    };

    let serve_file_path = format!("{}{}", base_path, filename);
    let path: PathBuf = PathBuf::from(serve_file_path);
    Ok(NamedFile::open(path)?)
}

pub fn healthcheck(_req: HttpRequest) -> HttpResponse {
    let message = HealthCheckResponse {
        app_name: APP_NAME.to_string(),
        message: "Everything's fine !".to_string(),
    };

    HttpResponse::Ok().json(message)
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
