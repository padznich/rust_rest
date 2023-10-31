
use actix_web::{web, HttpResponse};

use crate::db::tutor::{get_all_tutors_db, get_tutor_details_db, post_new_tutor_db, delete_tutor_db, update_tutor_details_db};
use crate::errors::EzyTutorError;
use crate::models::tutor::{CreateTutor, UpdateTutor};
use crate::state::AppState;


pub async fn get_all_tutors(app_state: web::Data<AppState>) -> Result<HttpResponse, EzyTutorError> {
    get_all_tutors_db(&app_state.db)
        .await
        .map(|tutors| HttpResponse::Ok().json(tutors))
}


pub async fn get_tutor_details(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, EzyTutorError> {

    let id: i32 = params.into_inner();

    get_tutor_details_db(&app_state.db, id)
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}


pub async fn post_new_tutor(
    new_tutor: web::Json<CreateTutor>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, EzyTutorError> { post_new_tutor_db(&app_state.db, CreateTutor::from(new_tutor))
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}


pub async fn update_tutor_details(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
    payload: web::Json<UpdateTutor>,
) -> Result<HttpResponse, EzyTutorError> {

    let id: i32 = params.into_inner();

    update_tutor_details_db(&app_state.db, id, UpdateTutor::from(payload))
      .await
      .map(|tutor| HttpResponse::Ok().json(tutor))
}


pub async fn delete_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, EzyTutorError> {
    let id: i32 = params.into_inner();

    delete_tutor_db(&app_state.db, id)
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}
