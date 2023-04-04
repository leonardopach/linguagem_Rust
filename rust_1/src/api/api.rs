use actix_web::*;

use crate::api::catalogo;
use crate::api::info;
use crate::api::ping;

#[actix_web::main]
pub async fn api_request() -> Result<(), std::io::Error> {
    let api = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(ping))
            .route("/info", web::get().to(info))
            .route("/cat", web::get().to(catalogo))
    });

    let porta = 9091;

    let api = api
        .bind(format!("127.0.0.1:{}", porta))
        .expect("Nâo conseguiu conectar...");

    println!("Conectado com sucesso! \n http://localhost:{}", porta);

    api.run().await
}

pub fn print_api() {
    let _ = api_request();
}
