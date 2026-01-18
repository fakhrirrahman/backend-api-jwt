use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize };
use validator::Validate;



#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, message = "Nama minimal 3 karakter" ))]
    pub name: String,

    #[validate(email(message = "email tidak ditemukan"))]
    pub email: String,

    #[validate(length(min = 6, message = "password minimal 6 karakter"))]
    pub password: String,

}
#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterResponse {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}