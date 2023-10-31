// use std::convert::TryFrom;

use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// use crate::errors::EzyTutorError;


#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Course {
    pub id: i32,
    pub tutor_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
    pub posted_time: Option<NaiveDateTime>,
}


/////////////////////////////////////////
// Course CREATE
/////////////////////////////////////////
#[derive(Deserialize, Debug, Clone)]
pub struct CreateCourse {
    pub tutor_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

impl From<web::Json<CreateCourse>> for CreateCourse {
    fn from(_course: web::Json<CreateCourse>) -> Self {
        CreateCourse {
            tutor_id: _course.tutor_id,
            name: _course.name.clone(),
            description: _course.description.clone(),
            format: _course.format.clone(),
            structure: _course.structure.clone(),
            level: _course.level.clone(),
            duration: _course.duration.clone(),
            language: _course.language.clone(),
            price: _course.price,
        }
    }
}

// impl TryFrom<web::Json<CreateCourse>> for CreateCourse { type Error = EzyTutorError;
//     fn try_from(_course: web::Json<CreateCourse>) ->
//       Result<Self, Self::Error> {
//         Ok(CreateCourse {
//             tutor_id: _course.tutor_id,
//             name: _course.name.clone(),
//             description: _course.description.clone(),
//             format: _course.format.clone(),
//             structure: _course.structure.clone(),
//             level: _course.level.clone(),
//             duration: _course.duration.clone(),
//             language: _course.language.clone(),
//             price: _course.price
//         })
//     }
// }



/////////////////////////////////////////
// Course UPDATE
/////////////////////////////////////////
#[derive(Deserialize, Debug, Clone)]
pub struct UpdateCourse {
    pub name: Option<String>,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

impl From<web::Json<UpdateCourse>> for UpdateCourse {
    fn from(update_course: web::Json<UpdateCourse>) -> Self {
        UpdateCourse {
            name: update_course.name.clone(),
            description: update_course.description.clone(),
            format: update_course.format.clone(),
            structure: update_course.structure.clone(),
            level: update_course.level.clone(),
            duration: update_course.duration.clone(),
            language: update_course.language.clone(),
            price: update_course.price
        }
    }
}