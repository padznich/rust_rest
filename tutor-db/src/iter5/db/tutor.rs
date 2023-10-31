use crate::errors::EzyTutorError;
use crate::models::tutor::{CreateTutor, Tutor, UpdateTutor};
use sqlx::postgres::PgPool;



pub async fn get_all_tutors_db(pool: &PgPool) ->
  Result<Vec<Tutor>, EzyTutorError> {
    // Prepare SQL statement
    let tutor_rows =
        sqlx::query!("SELECT id, name, pic_url, profile FROM ezy_tutor")
            .fetch_all(pool)
            .await?;
    // Extract result
    let tutors: Vec<Tutor> = tutor_rows
        .iter()
        .map(|tutor_row| Tutor {
            id: tutor_row.id,
            name: tutor_row.name.clone(),
            pic_url: tutor_row.pic_url.clone(),
            profile: tutor_row.profile.clone()
        })
        .collect();

    match tutors.len() {
        0 => Err(EzyTutorError::NotFound("No tutors found".into())),
        _ => Ok(tutors),
    }
}


pub async fn get_tutor_details_db(pool: &PgPool, id: i32) -> Result<Tutor, EzyTutorError> {
    // Prepare SQL statement
    let tutor_row = sqlx::query!(
        "SELECT id, name, pic_url, profile FROM ezy_tutor where id = $1",
    id )
        .fetch_one(pool)
        .await
        .map(|tutor_row|
            Tutor {
                id: tutor_row.id,
                name: tutor_row.name,
                pic_url: tutor_row.pic_url,
                profile: tutor_row.profile
            } )
        .map_err(|_err| EzyTutorError::NotFound("Tutor not found".into()))?; Ok(tutor_row)
}


pub async fn post_new_tutor_db(pool: &PgPool, new_tutor: CreateTutor) -> Result<Tutor, EzyTutorError> {
    let tutor_row = sqlx::query!("insert into ezy_tutor ( name, pic_url, profile) values ($1,$2,$3) returning id, name, pic_url, profile",
        new_tutor.name,
        new_tutor.pic_url,
        new_tutor.profile
    )
        .fetch_one(pool)
        .await?;

    //Retrieve result
    Ok(Tutor {
        id: tutor_row.id,
        name: tutor_row.name,
        pic_url: tutor_row.pic_url,
        profile: tutor_row.profile,
    })
}


pub async fn delete_tutor_db(
    pool: &PgPool,
    id: i32,
) -> Result<String, EzyTutorError> {

    let row = sqlx::query!(
        "DELETE FROM ezy_tutor where id = $1",
        id,
    )
        .execute(pool)
        .await?;

    Ok(format!("Deleted {:#?} record", row))
}


pub async fn update_tutor_details_db(
    pool: &PgPool,
    id: i32,
    payload: UpdateTutor,
) -> Result<Tutor, EzyTutorError> {

    let current_tutor_row = sqlx::query_as!(
        Tutor,
        "SELECT * FROM ezy_tutor where id = $1", id
    )
        .fetch_one(pool)
        .await
        .map_err(|_err| EzyTutorError::NotFound("Tutor id not found".into()))?;

    let name: String = if let Some(name) = payload.name {
        name
    } else {
        current_tutor_row.name
    };

    let pic_url: String = if let Some(pic_url) = payload.pic_url {
        pic_url
    } else {
        current_tutor_row.pic_url
    };

    let profile: String = if let Some(profile) = payload.profile {
        profile
    } else {
        current_tutor_row.profile
    };

    // Prepare SQL statement
    let tutor_row = sqlx::query_as!(
        Tutor,
        "UPDATE
            ezy_tutor
        SET
            name = $1,
            pic_url = $2,
            profile = $3
        WHERE
            id = $4
        RETURNING
            id,
            name,
            pic_url,
            profile",
        name,
        pic_url,
        profile,
        id
    )
        .fetch_one(pool)
        .await;

    if let Ok(tutor) = tutor_row {
        Ok(tutor)
    } else {
        Err(EzyTutorError::NotFound("Tutor not found".into()))
    }
}

