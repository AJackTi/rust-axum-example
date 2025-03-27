use serde::{ Deserialize, Serialize };
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, FromRow, Debug, ToSchema)]
pub struct Part {
    pub id: i32,
    pub car_id: i32,
    pub name: String,
}

pub type PartList = Vec<Part>;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct NewPart {
    pub car_id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartQuery {
    pub name: Option<String>,
}
