pub mod app_data {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct AppData {
        pub dude: String,
    }
}

mod api {
    use super::app_data::AppData;
    use actix_web::{get, web, HttpResponse, Responder, Scope};

    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct GetPayload {
        pub message: String,
    }

    #[get("/hi")]
    async fn github_authorize_url(config: web::Data<AppData>) -> impl Responder {
        let name = config.dude.clone();
        HttpResponse::Ok().json(GetPayload {
            message: format!("Hi ${}", name),
        })
    }

    pub fn scope() -> Scope {
        web::scope("/v1/say").service(github_authorize_url)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::middleware::Logger;
    use app_data::AppData;
    use std::sync::Arc;

    let data = Arc::new(AppData {
        dude: "Dude".to_string(),
    });
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(Logger::default())
            .data(data.clone())
            .service(api::scope())
    })
    .bind("0.0.0.0:7070")?
    .run()
    .await
}
