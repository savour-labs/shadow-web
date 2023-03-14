use poem::web::Data;
use poem_openapi::{
    payload::Json,
    ApiResponse, OpenApi,
};
use sqlx::PgPool;

use crate::{
    core,
    entities::dapp::{ Dapp },
};

use super::ApiTags;
pub struct DappAPI;

#[OpenApi]
impl DappAPI {
    #[oai(path = "/api/dapps", method = "get", tag = "ApiTags::Dapp")]
    async fn get_dapps(&self) -> GetDappsResponse {
         let dapp = Dapp {
            title: String::from("Shadow-X"),
            description: String::from("Shadow-X 666"),
        };
        GetDappsResponse::Ok(Json(dapp))
    }
}

#[derive(ApiResponse)]
pub enum GetDappsResponse {
    #[oai(status = 200)]
    Ok(Json<Dapp>),

    #[oai(status = 500)]
    Internal,
}


