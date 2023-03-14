use poem::{ Route};
use poem_openapi::{OpenApiService, Tags};

pub mod dapp;
pub mod download;
pub mod roadmap;
pub mod social;

#[derive(Tags)]
enum ApiTags {
    Dapp,
    Download,
    Social,
}

pub fn routes() -> Route {
    let openapi_service = OpenApiService::new(
        (dapp::DappAPI),
        "Let's Science API",
        "0.1",
    ).server("http://localhost:3000/api");
     Route::new()
         .nest_no_strip("/api", openapi_service)
}