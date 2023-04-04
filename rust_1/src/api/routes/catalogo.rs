use actix_web::*;

pub async fn catalogo() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body(
            r#"[
            {"mensagem":"olá mundo","autor": "Leonardo", "email": "derpleo@gmail.com"}]"#,
        )
}
