use sqlx::postgres::PgPool;

use crate::errors::EzyTutorError;
use crate::models::course::*;


pub async fn get_courses_for_tutor_db(pool: &PgPool, tutor_id: i32) -> Result<Vec<Course>, EzyTutorError> {
    let course_rows: Vec<Course> = sqlx::query_as!(
        Course,
        "SELECT * FROM ezy_course where tutor_id = $1",
        tutor_id
    )
        .fetch_all(pool)
        .await?;

    Ok(course_rows)
}


pub async fn get_course_details_db(pool: &PgPool, tutor_id: i32, course_id: i32) -> Result<Course, EzyTutorError> {

    let course_row = sqlx::query_as!(
        Course,
        "SELECT * FROM ezy_course where tutor_id = $1 and id = $2",
        tutor_id,
        course_id
    )
        .fetch_optional(pool)
        .await?;

    if let Some(course) = course_row {
        Ok(course)
    } else {
        Err(EzyTutorError::NotFound("Course id not found".into()))
    }
}


pub async fn post_new_course_db(pool: &PgPool, new_course: CreateCourse) -> Result<Course, EzyTutorError> {

    let course_row= sqlx::query_as!(
        Course,
        "insert into ezy_course (
             tutor_id,
             name,
             description,
             duration,
             level,
             format,
             language,
             structure,
             price
        ) values ($1,$2,$3,$4,$5,$6,$7,$8,$9)
        returning
            tutor_id,
            id,
            name,
            description,
            duration,
            level,
            format,
            language,
            structure,
            price,
            posted_time",
        new_course.tutor_id,
        new_course.name,
        new_course.description,
        new_course.duration,
        new_course.level,
        new_course.format,
        new_course.language,
        new_course.structure,
        new_course.price
    )
    .fetch_one(pool)
    .await?;

    Ok(course_row)
}


pub async fn delete_course_db(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
) -> Result<String, EzyTutorError> {

    let course_row = sqlx::query!(
        "DELETE FROM ezy_course where tutor_id = $1 and id = $2",
        tutor_id,
        course_id,
    )
        .execute(pool)
        .await?;

    Ok(format!("Deleted {:#?} record", course_row))
}


pub async fn update_course_details_db(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
    update_course: UpdateCourse,
) -> Result<Course, EzyTutorError> {

    let current_course_row = sqlx::query_as!(
        Course,
        "SELECT * FROM ezy_course where tutor_id = $1 and id = $2", tutor_id, course_id
    )
        .fetch_one(pool)
        .await
        .map_err(|_err| EzyTutorError::NotFound("Course id not found".into()))?;

    let name: String = if let Some(name) = update_course.name {
        name
    } else {
        current_course_row.name
    };

    let description: String = if let Some(description) = update_course.description {
        description
    } else {
        current_course_row.description.unwrap_or_default()
    };

    let format: String = if let Some(format) = update_course.format {
        format
    } else {
        current_course_row.format.unwrap_or_default()
    };

    let structure: String = if let Some(structure) = update_course.structure {
        structure
    } else {
        current_course_row.structure.unwrap_or_default()
    };

    let duration: String = if let Some(duration) = update_course.duration {
        duration
    } else {
        current_course_row.duration.unwrap_or_default()
    };

    let level: String = if let Some(level) = update_course.level {
        level
    } else {
        current_course_row.level.unwrap_or_default()
    };

    let language: String = if let Some(language) = update_course.language {
        language
    } else {
        current_course_row.language.unwrap_or_default()
    };

    let price: i32 = if let Some(price) = update_course.price {
        price
    } else {
        current_course_row.price.unwrap_or_default()
    };

    // Prepare SQL statement
    let course_row = sqlx::query_as!(
        Course,
        "UPDATE
            ezy_course
        SET
            name = $1,
            description = $2,
            format = $3,
            structure = $4,
            duration = $5,
            price = $6,
            language = $7,
            level = $8
        WHERE
            tutor_id = $9 AND id = $10
        RETURNING
            tutor_id,
            id,
            name,
            description,
            duration,
            level,
            format,
            language,
            structure,
            price,
            posted_time",
        name,
        description,
        format,
        structure,
        duration,
        price,
        language,
        level,
        tutor_id,
        course_id
    )
        .fetch_one(pool)
        .await;

    if let Ok(course) = course_row {
        Ok(course)
    } else {
        Err(EzyTutorError::NotFound("Course id not found".into()))
    }
}
