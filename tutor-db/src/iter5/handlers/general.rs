use actix_web::{web, HttpResponse, error::Error};

use crate::state::AppState;


pub async fn health_check_handler(app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);

    *visit_count += 1;

    Ok(HttpResponse::Ok().json(&response))
}
