use actix_web::web;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Tutor {
    pub id: i32,
    pub name: String,
    pub pic_url: String,
    pub profile: String
}


/////////////////////////////////////////
// Tutor CREATE
/////////////////////////////////////////
#[derive(Deserialize, Debug, Clone)]
pub struct CreateTutor {
    pub name: String,
    pub pic_url: String,
    pub profile: String,
}

impl From<web::Json<CreateTutor>> for CreateTutor {
    fn from(new_tutor: web::Json<CreateTutor>) -> Self {
        CreateTutor {
            name: new_tutor.name.clone(),
            pic_url: new_tutor.pic_url.clone(),
            profile: new_tutor.profile.clone()
        }
    }
}


/////////////////////////////////////////
// Tutor UPDATE
/////////////////////////////////////////
#[derive(Deserialize, Debug, Clone)]
pub struct UpdateTutor {
    pub name: Option<String>,
    pub pic_url: Option<String>,
    pub profile: Option<String>,
}


impl From<web::Json<UpdateTutor>> for UpdateTutor {
    fn from(new_tutor: web::Json<UpdateTutor>) -> Self {
        UpdateTutor {
            name: new_tutor.name.clone(),
            pic_url: new_tutor.pic_url.clone(),
            profile: new_tutor.profile.clone()
        }
    }
}
